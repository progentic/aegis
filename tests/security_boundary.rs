use aegis_authority::limits::{
    MAX_COMPONENT_BYTES, MAX_DOCUMENT_BYTES, MAX_IDENTITY_BYTES, MAX_PATH_BYTES, MAX_SET_ENTRIES,
};
use aegis_authority::{
    Capability, ContractError, PolicyDocument, TaskDocument, parse_policy, parse_task,
    validate_contract,
};
use serde_json::{Value, json};

const POLICY: &str = include_str!("fixtures/policy.json");
const TASK: &str = include_str!("fixtures/task.json");
const MARKER: &str = "sensitive-value-that-must-not-appear-in-diagnostics";

#[test]
fn documents_at_the_byte_limit_are_accepted() {
    assert!(parse_policy(&padded_document(POLICY, MAX_DOCUMENT_BYTES)).is_ok());
    assert!(parse_task(&padded_document(TASK, MAX_DOCUMENT_BYTES)).is_ok());
}

#[test]
fn oversized_documents_are_rejected_before_json_parsing() {
    let text = " ".repeat(MAX_DOCUMENT_BYTES + 1);
    assert_eq!(parse_policy(&text), Err(ContractError::DocumentTooLarge));
    assert_eq!(parse_task(&text), Err(ContractError::DocumentTooLarge));
}

#[test]
fn oversized_policy_sets_are_rejected_by_parsing_and_direct_validation() {
    for pointer in policy_sets() {
        let text = oversized_set(POLICY, pointer);
        assert_eq!(parse_policy(&text), Err(ContractError::TooManyEntries));
        let policy: PolicyDocument = serde_json::from_str(&text).unwrap();
        let task = parse_task(TASK).unwrap();
        assert_eq!(
            validate_contract(&policy, &task, &task),
            Err(ContractError::TooManyEntries)
        );
    }
}

#[test]
fn oversized_task_sets_are_rejected_by_parsing_and_direct_validation() {
    for pointer in task_sets() {
        let text = oversized_set(TASK, pointer);
        assert_eq!(parse_task(&text), Err(ContractError::TooManyEntries));
        let task: TaskDocument = serde_json::from_str(&text).unwrap();
        let policy = parse_policy(POLICY).unwrap();
        assert_eq!(
            validate_contract(&policy, &task, &task),
            Err(ContractError::TooManyEntries)
        );
    }
}

#[test]
fn full_sets_of_distinct_paths_are_accepted() {
    let mut paths: Vec<_> = (0..MAX_SET_ENTRIES - 1)
        .map(|n| format!("src/{n}.rs"))
        .collect();
    paths.push(String::from("tests/new.rs"));
    let policy = changed(POLICY, "/paths/create", json!(paths));
    assert!(parse_policy(&policy).is_ok());
}

#[test]
fn identity_limits_count_utf8_bytes() {
    let at_limit = "é".repeat(MAX_IDENTITY_BYTES / 2);
    assert!(parse_task(&changed(TASK, "/scope/task_id", json!(at_limit))).is_ok());
    let too_long = format!("{at_limit}a");
    assert_eq!(
        parse_task(&changed(TASK, "/scope/task_id", json!(too_long))),
        Err(ContractError::IdentityTooLong)
    );
}

#[test]
fn identity_length_is_rechecked_for_direct_inputs() {
    let policy = parse_policy(POLICY).unwrap();
    let mut task = parse_task(TASK).unwrap();
    task.scope.approval_reference = "a".repeat(MAX_IDENTITY_BYTES + 1);
    assert_eq!(
        validate_contract(&policy, &task, &task),
        Err(ContractError::IdentityTooLong)
    );
}

#[test]
fn ambiguous_or_control_bearing_identity_fields_are_rejected() {
    for value in [
        "owner\nforged",
        "\0owner",
        "owner\u{202e}",
        "owner\u{061c}",
        "owner\u{200e}",
        "owner\u{200f}",
        " owner",
        "owner ",
    ] {
        let text = changed(TASK, "/scope/policy/owner", json!(value));
        assert_eq!(parse_task(&text), Err(ContractError::InvalidIdentity));
    }
}

#[test]
fn path_controls_are_rejected_in_scopes_and_protected_targets() {
    for value in ["src/a\n.rs", "src/\u{1b}[31ma.rs", "src/\u{2066}a.rs"] {
        for pointer in [
            "/scope/paths/read/0",
            "/scope/capabilities/4/ProtectedAction/target",
        ] {
            let text = changed(TASK, pointer, json!(value));
            assert_eq!(parse_task(&text), Err(ContractError::InvalidPath));
        }
    }
}

#[test]
fn path_and_component_limits_are_enforced() {
    let path = format!("{}a", "a/".repeat((MAX_PATH_BYTES - 1) / 2));
    assert!(parse_task(&with_path(TASK, &path)).is_ok());
    assert_eq!(
        parse_task(&with_path(TASK, &format!("{path}a"))),
        Err(ContractError::InvalidPath)
    );
    assert!(parse_task(&with_path(TASK, &"a".repeat(MAX_COMPONENT_BYTES))).is_ok());
    assert_eq!(
        parse_task(&with_path(TASK, &"a".repeat(MAX_COMPONENT_BYTES + 1))),
        Err(ContractError::InvalidPath)
    );
}

#[test]
fn multibyte_components_obey_the_same_byte_limit() {
    let component = "é".repeat(MAX_COMPONENT_BYTES / 2);
    assert!(parse_policy(&with_path(POLICY, &format!("{component}a"))).is_ok());
    assert_eq!(
        parse_policy(&with_path(POLICY, &format!("{component}é"))),
        Err(ContractError::InvalidPath)
    );
}

#[test]
fn unsupported_secret_network_and_process_capabilities_fail_closed() {
    for name in [
        "AccessSecret",
        "Network",
        "InvokeProcess",
        "Execute",
        "Permit",
        "ALLOW",
    ] {
        let text = changed(TASK, "/scope/capabilities", json!([{(name): MARKER}]));
        assert_eq!(parse_task(&text), Err(ContractError::Json));
    }
}

#[test]
fn credential_and_caller_created_authority_fields_are_not_accepted() {
    for name in [
        "credentials",
        "environment",
        "api_key",
        "command",
        "permit",
        "decision",
    ] {
        let mut value: Value = serde_json::from_str(TASK).unwrap();
        value["scope"][name] = json!(MARKER);
        assert_eq!(parse_task(&value.to_string()), Err(ContractError::Json));
    }
}

#[test]
fn diagnostics_never_echo_rejected_values() {
    for text in diagnostic_payloads() {
        let error = parse_task(&text).unwrap_err();
        assert!(!format!("{error} {error:?}").contains(MARKER));
        assert!(error.code().starts_with("CONTRACT_"));
    }
}

#[test]
fn debug_output_redacts_authority_inputs_and_validated_snapshots() {
    let policy_text = POLICY.replace("human-owner", MARKER);
    let task_text = TASK.replace("human-owner", MARKER);
    let policy = parse_policy(&policy_text).unwrap();
    let task = parse_task(&task_text).unwrap();
    let contract = validate_contract(&policy, &task, &task).unwrap();
    assert!(!format!("{policy:?} {task:?} {contract:?}").contains(MARKER));
    assert!(!format!("{:?}", task.scope.policy).contains(MARKER));
}

#[test]
fn debug_output_redacts_paths_and_capability_targets() {
    let task = parse_task(&with_path(TASK, MARKER)).unwrap();
    let values = format!(
        "{:?} {:?} {:?}",
        task.scope, task.scope.paths, task.scope.capabilities
    );
    assert!(!values.contains(MARKER));
    let grant = changed(
        TASK,
        "/scope/capabilities/4/ProtectedAction/target",
        json!(MARKER),
    );
    assert!(!format!("{:?}", parse_task(&grant).unwrap().scope.capabilities).contains(MARKER));
}

#[test]
fn directly_formatted_protected_grants_do_not_disclose_their_target() {
    let text = changed(
        TASK,
        "/scope/capabilities/4/ProtectedAction/target",
        json!(MARKER),
    );
    let task = parse_task(&text).unwrap();
    let Capability::ProtectedAction(grant) = &task.scope.capabilities[4] else {
        panic!("expected protected grant fixture");
    };
    assert!(!format!("{grant:?}").contains(MARKER));
}

#[test]
fn snapshots_retain_the_policy_and_do_not_alias_mutable_inputs() {
    let mut policy = parse_policy(POLICY).unwrap();
    let mut task = parse_task(TASK).unwrap();
    let contract = validate_contract(&policy, &task, &task).unwrap();
    policy.capabilities.clear();
    task.scope.capabilities.clear();
    assert!(!contract.policy().capabilities.is_empty());
    assert!(!contract.task().scope.capabilities.is_empty());
}

#[test]
fn changed_policy_contents_cannot_reuse_the_same_validated_snapshot() {
    let mut policy = parse_policy(POLICY).unwrap();
    let task = parse_task(TASK).unwrap();
    let original = validate_contract(&policy, &task, &task).unwrap();
    policy.paths.read.push(String::from("src/another.rs"));
    let changed = validate_contract(&policy, &task, &task).unwrap();
    assert_eq!(original.policy().policy, changed.policy().policy);
    assert_ne!(original, changed);
}

#[test]
fn reordered_policy_sets_preserve_the_validated_snapshot() {
    let mut policy = parse_policy(POLICY).unwrap();
    let task = parse_task(TASK).unwrap();
    let original = validate_contract(&policy, &task, &task).unwrap();
    policy.paths.read.reverse();
    policy.capabilities.reverse();
    policy.invariants.reverse();
    assert_eq!(validate_contract(&policy, &task, &task).unwrap(), original);
}

#[test]
fn deeply_nested_json_is_rejected_without_a_fallback() {
    let text = format!("{}null{}", "[".repeat(256), "]".repeat(256));
    assert_eq!(parse_policy(&text), Err(ContractError::Json));
    assert_eq!(parse_task(&text), Err(ContractError::Json));
}

fn padded_document(text: &str, bytes: usize) -> String {
    format!("{text}{}", " ".repeat(bytes - text.len()))
}

fn policy_sets() -> [&'static str; 5] {
    [
        "/paths/read",
        "/paths/write",
        "/paths/create",
        "/capabilities",
        "/invariants",
    ]
}

fn task_sets() -> [&'static str; 5] {
    [
        "/scope/paths/read",
        "/scope/paths/write",
        "/scope/paths/create",
        "/scope/capabilities",
        "/scope/invariants",
    ]
}

fn oversized_set(text: &str, pointer: &str) -> String {
    let value: Value = serde_json::from_str(text).unwrap();
    let entry = value.pointer(pointer).unwrap()[0].clone();
    changed(text, pointer, json!(vec![entry; MAX_SET_ENTRIES + 1]))
}

fn diagnostic_payloads() -> [String; 4] {
    [
        TASK.replace("schema_version", MARKER),
        changed(TASK, "/scope/capabilities", json!([{MARKER: MARKER}])),
        changed(TASK, "/scope/paths/read/0", json!(format!("../{MARKER}"))),
        changed(TASK, "/scope/invariants", json!([MARKER])),
    ]
}

fn with_path(text: &str, path: &str) -> String {
    text.replace("src/a.rs", path)
}

fn changed(text: &str, pointer: &str, replacement: Value) -> String {
    let mut value: Value = serde_json::from_str(text).unwrap();
    *value.pointer_mut(pointer).unwrap() = replacement;
    value.to_string()
}
