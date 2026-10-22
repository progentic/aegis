use aegis_authority::{
    Capability, ContractError, PolicyDocument, TaskDocument, parse_policy, parse_task,
    validate_contract,
};
use serde_json::{Value, json};

const POLICY: &str = include_str!("fixtures/policy.json");
const TASK: &str = include_str!("fixtures/task.json");

#[test]
fn valid_contract_is_deterministic_and_does_not_mutate_inputs() {
    let (policy, task) = inputs();
    let before = task.clone();
    let first = validate_contract(&policy, &task, &task).unwrap();
    let second = validate_contract(&policy, &task, &task).unwrap();
    assert_eq!(first, second);
    assert_eq!(task, before);
    assert_eq!(policy, parse_policy(POLICY).unwrap());
    assert_eq!(first.task().scope.paths.read, ["src/a.rs", "src/b.rs"]);
}

#[test]
fn reordered_sets_produce_the_same_contract() {
    let (policy, approved) = inputs();
    let mut task = approved.clone();
    task.scope.paths.read.reverse();
    task.scope.capabilities.reverse();
    task.scope.invariants.reverse();
    assert_eq!(
        validate_contract(&policy, &task, &approved).unwrap(),
        validate_contract(&policy, &approved, &approved).unwrap()
    );
}

#[test]
fn every_required_json_field_is_required() {
    check_required_fields(POLICY, parse_policy);
    check_required_fields(TASK, parse_task);
}

#[test]
fn unknown_fields_at_each_object_level_are_rejected() {
    check_unknown_fields(POLICY, parse_policy);
    check_unknown_fields(TASK, parse_task);
}

#[test]
fn empty_explicit_grants_remain_empty() {
    let (mut policy, mut task) = inputs();
    clear_grants(&mut policy, &mut task);
    let result = validate_contract(&policy, &task, &task).unwrap();
    assert!(result.task().scope.capabilities.is_empty());
    assert!(result.task().scope.paths.write.is_empty());
}

#[test]
fn malformed_json_and_duplicate_keys_are_rejected() {
    for text in malformed_documents() {
        assert!(parse_policy(&text).is_err(), "policy accepted {text}");
        assert!(parse_task(&text).is_err(), "task accepted {text}");
    }
}

#[test]
fn policy_mutations_are_rejected() {
    for (pointer, value) in policy_mutations() {
        let text = changed(POLICY, pointer, value);
        assert!(parse_policy(&text).is_err(), "accepted {pointer}: {text}");
    }
}

#[test]
fn task_mutations_are_rejected() {
    for (pointer, value) in task_mutations() {
        let text = changed(TASK, pointer, value);
        assert!(parse_task(&text).is_err(), "accepted {pointer}: {text}");
    }
}

#[test]
fn duplicate_keys_at_each_object_level_are_rejected() {
    for (needle, replacement) in duplicate_fields() {
        let text = TASK.replace(needle, replacement);
        assert!(parse_task(&text).is_err(), "accepted {text}");
    }
}

#[test]
fn invalid_paths_are_rejected_in_scopes_and_capabilities() {
    for path in invalid_paths() {
        let scope = changed(TASK, "/scope/paths/read/0", json!(path));
        let capability = changed(TASK, "/scope/capabilities/0/WritePath", json!(path));
        assert!(parse_task(&scope).is_err(), "accepted scope path {path:?}");
        assert!(
            parse_task(&capability).is_err(),
            "accepted capability {path:?}"
        );
    }
}

#[test]
fn policy_reference_or_repository_mismatch_is_rejected() {
    for pointer in binding_fields() {
        let (policy, _) = inputs();
        let task = parse_task(&changed(TASK, pointer, json!("different"))).unwrap();
        assert_eq!(
            validate_contract(&policy, &task, &task),
            Err(ContractError::Mismatch)
        );
    }
}

#[test]
fn approval_is_bound_to_the_entire_scope() {
    let (policy, approved) = inputs();
    let mut task = approved.clone();
    task.scope.paths.read.clear();
    task.scope
        .capabilities
        .retain(|cap| !matches!(cap, Capability::ReadPath(_)));
    assert_eq!(
        validate_contract(&policy, &task, &approved),
        Err(ContractError::Mismatch)
    );
}

#[test]
fn reused_approval_reference_cannot_authorize_expansion() {
    let (policy, task) = inputs();
    let mut approved = task.clone();
    approved.scope.paths.create.clear();
    approved
        .scope
        .capabilities
        .retain(|cap| !matches!(cap, Capability::CreatePath(_)));
    assert_eq!(
        validate_contract(&policy, &task, &approved),
        Err(ContractError::Mismatch)
    );
}

#[test]
fn requests_outside_the_policy_ceiling_are_rejected() {
    for pointer in ceiling_fields() {
        let paths = changed(POLICY, pointer, json!([]));
        let text = changed(&paths, "/capabilities", json!([]));
        let policy = parse_policy(&text).unwrap();
        let task = parse_task(TASK).unwrap();
        assert_eq!(
            validate_contract(&policy, &task, &task),
            Err(ContractError::OutsidePolicy)
        );
    }
}

#[test]
fn protected_capabilities_require_the_same_category_and_target() {
    for pointer in [
        "/capabilities/4/ProtectedAction/action",
        "/capabilities/4/ProtectedAction/target",
    ] {
        let replacement = protected_replacement(pointer);
        let policy = parse_policy(&changed(POLICY, pointer, replacement)).unwrap();
        let task = parse_task(TASK).unwrap();
        assert_eq!(
            validate_contract(&policy, &task, &task),
            Err(ContractError::OutsidePolicy)
        );
    }
}

#[test]
fn blank_provenance_is_rejected_even_for_direct_inputs() {
    let (policy, original) = inputs();
    for pointer in [
        "/scope/task_id",
        "/scope/approval_reference",
        "/scope/policy/owner",
        "/scope/policy/version",
        "/scope/policy/reference",
    ] {
        let task: TaskDocument = serde_json::from_str(&changed(TASK, pointer, json!(""))).unwrap();
        assert_eq!(
            validate_contract(&policy, &task, &original),
            Err(ContractError::MissingIdentity)
        );
    }
}

#[test]
fn required_policy_invariants_cannot_be_removed() {
    let (policy, mut task) = inputs();
    task.scope.invariants.clear();
    assert_eq!(
        validate_contract(&policy, &task, &task),
        Err(ContractError::MissingRequiredInvariant)
    );
}

#[test]
fn validation_rechecks_directly_constructed_inputs() {
    let (policy, mut task) = inputs();
    task.scope.approval_reference.clear();
    assert_eq!(
        validate_contract(&policy, &task, &task),
        Err(ContractError::MissingIdentity)
    );
}

#[test]
fn ordinary_capabilities_cannot_substitute_for_a_protected_grant() {
    let (mut policy, task) = inputs();
    policy
        .capabilities
        .retain(|cap| !matches!(cap, Capability::ProtectedAction(_)));
    assert_eq!(
        validate_contract(&policy, &task, &task),
        Err(ContractError::OutsidePolicy)
    );
}

#[test]
fn path_operations_do_not_imply_each_other() {
    for granted in path_operations() {
        for requested in path_operations() {
            let (mut policy, mut task) = inputs();
            equal_path_bounds(&mut policy, &mut task);
            policy.capabilities = vec![granted.clone()];
            task.scope.capabilities = vec![requested.clone()];
            let result = validate_contract(&policy, &task, &task);
            assert_eq!(result.is_ok(), granted == requested);
        }
    }
}

#[test]
fn path_bounds_are_exact_and_case_sensitive() {
    for path in [
        "src/A.rs",
        "src/a.rs/extra",
        "src/a.rs_evil",
        "src/other.rs",
    ] {
        let (policy, mut task) = inputs();
        task.scope.paths.write = vec![String::from(path)];
        task.scope.capabilities = vec![Capability::WritePath(String::from(path))];
        assert_eq!(
            validate_contract(&policy, &task, &task),
            Err(ContractError::OutsidePolicy)
        );
    }
}

#[test]
fn rejected_requests_leave_inputs_unchanged() {
    let (policy, mut task) = inputs();
    task.scope.invariants.push(String::from("INV-UNKNOWN"));
    let before = task.clone();
    assert!(validate_contract(&policy, &task, &task).is_err());
    assert_eq!(task, before);
    assert_eq!(policy, parse_policy(POLICY).unwrap());
}

fn inputs() -> (PolicyDocument, TaskDocument) {
    (parse_policy(POLICY).unwrap(), parse_task(TASK).unwrap())
}

fn path_operations() -> Vec<Capability> {
    vec![
        Capability::ReadPath(String::from("src/a.rs")),
        Capability::WritePath(String::from("src/a.rs")),
        Capability::CreatePath(String::from("src/a.rs")),
    ]
}

fn equal_path_bounds(policy: &mut PolicyDocument, task: &mut TaskDocument) {
    policy.paths.read = vec![String::from("src/a.rs")];
    policy.paths.write = policy.paths.read.clone();
    policy.paths.create = policy.paths.read.clone();
    task.scope.paths = policy.paths.clone();
}

fn clear_grants(policy: &mut PolicyDocument, task: &mut TaskDocument) {
    policy.capabilities.clear();
    task.scope.capabilities.clear();
    policy.paths.read.clear();
    policy.paths.write.clear();
    policy.paths.create.clear();
    task.scope.paths = policy.paths.clone();
}

fn check_unknown_fields<T>(text: &str, parser: fn(&str) -> Result<T, ContractError>) {
    let original: Value = serde_json::from_str(text).unwrap();
    for pointer in object_pointers(&original) {
        let mut value = original.clone();
        value
            .pointer_mut(&pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert(String::from("unknown"), json!(true));
        assert!(
            parser(&value.to_string()).is_err(),
            "accepted unknown field at {pointer}"
        );
    }
}

fn protected_replacement(pointer: &str) -> Value {
    if pointer.ends_with("/action") {
        return json!("SchemaChange");
    }
    json!("other.toml")
}

fn check_required_fields<T>(text: &str, parser: fn(&str) -> Result<T, ContractError>) {
    let original: Value = serde_json::from_str(text).unwrap();
    for pointer in object_pointers(&original) {
        let keys: Vec<_> = original
            .pointer(&pointer)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        for key in keys {
            let mut value = original.clone();
            value
                .pointer_mut(&pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(&key);
            assert!(
                parser(&value.to_string()).is_err(),
                "accepted missing {pointer}/{key}"
            );
        }
    }
}

fn object_pointers(value: &Value) -> Vec<String> {
    let mut result = Vec::new();
    collect_objects(value, String::new(), &mut result);
    result
}

fn collect_objects(value: &Value, path: String, output: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            output.push(path.clone());
            for (key, child) in map {
                collect_objects(child, format!("{path}/{key}"), output);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                collect_objects(child, format!("{path}/{index}"), output);
            }
        }
        _ => {}
    }
}

fn changed(text: &str, pointer: &str, replacement: Value) -> String {
    let mut value: Value = serde_json::from_str(text).unwrap();
    *value.pointer_mut(pointer).unwrap() = replacement;
    value.to_string()
}

fn malformed_documents() -> Vec<String> {
    vec![
        String::from(""),
        String::from("null"),
        String::from("[]"),
        String::from("{}"),
        String::from("{bad}"),
        format!("{TASK} {{}}"),
        TASK.replace(
            "\"schema_version\": 1",
            "\"schema_version\": 1, \"schema_version\": 1",
        ),
        TASK.replace(
            "\"schema_version\": 1",
            "\"schema_version\": 1, \"schema_versio\\u006e\": 1",
        ),
    ]
}

fn policy_mutations() -> Vec<(&'static str, Value)> {
    vec![
        ("/schema_version", json!(2)),
        ("/repository_id", json!(" ")),
        ("/policy/owner", json!("")),
        ("/policy/version", json!(null)),
        ("/policy/reference", json!("")),
        ("/policy", json!(["x", "v", "r"])),
        ("/paths", json!([])),
        ("/paths/write", json!(null)),
        ("/paths/write", json!(["src/a.rs", "src/a.rs"])),
        ("/capabilities", json!([{"WritePath":"missing.rs"}])),
        ("/invariants", json!(["INV-UNKNOWN"])),
        ("/invariants", json!(["INV-007", "INV-007"])),
    ]
}

fn task_mutations() -> Vec<(&'static str, Value)> {
    vec![
        ("/schema_version", json!(0)),
        ("/scope", json!([])),
        ("/scope/task_id", json!("")),
        ("/scope/approval_reference", json!("\t")),
        ("/scope/policy", json!([])),
        ("/scope/paths", json!([])),
        ("/scope/capabilities", json!([{"Execute": "shell"}])),
        (
            "/scope/capabilities",
            json!([{"ReadPath":"src/a.rs"},{"ReadPath":"src/a.rs"}]),
        ),
        (
            "/scope/capabilities",
            json!([{"ProtectedAction":["DependencyChange", "Cargo.toml"]}]),
        ),
        (
            "/scope/capabilities",
            json!([{"ProtectedAction":{"action":{"DependencyChange":null},"target":"Cargo.toml"}}]),
        ),
        ("/scope/invariants", json!(["INV-UNKNOWN"])),
    ]
}

fn duplicate_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "\"task_id\": \"TASK-1\"",
            "\"task_id\": \"TASK-1\", \"task_id\": \"TASK-1\"",
        ),
        (
            "\"owner\": \"human-owner\"",
            "\"owner\": \"human-owner\", \"owner\": \"other\"",
        ),
        (
            "\"write\": [\"src/a.rs\"]",
            "\"write\": [], \"write\": [\"src/a.rs\"]",
        ),
        (
            "\"ReadPath\": \"src/b.rs\"",
            "\"ReadPath\": \"src/b.rs\", \"ReadPath\": \"src/a.rs\"",
        ),
        (
            "\"target\": \"Cargo.toml\"",
            "\"target\": \"Cargo.toml\", \"target\": \"other\"",
        ),
    ]
}

fn invalid_paths() -> Vec<&'static str> {
    vec![
        "",
        "/absolute",
        "../outside",
        "src/../outside",
        "./src/a.rs",
        "src//a.rs",
        "src/a.rs/",
        "src\\a.rs",
        "src/\0a.rs",
        "src/*.rs",
        "src/?.rs",
        "src/[ab].rs",
    ]
}

fn binding_fields() -> Vec<&'static str> {
    vec![
        "/repository_id",
        "/scope/policy/owner",
        "/scope/policy/version",
        "/scope/policy/reference",
    ]
}

fn ceiling_fields() -> Vec<&'static str> {
    vec![
        "/paths/read",
        "/paths/write",
        "/paths/create",
        "/capabilities",
    ]
}
