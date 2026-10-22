use crate::{Capability, PathScope, PolicyDocument, PolicyReference, TaskDocument, json};
use std::fmt;

const SCHEMA_VERSION: u32 = 1;
const KNOWN_INVARIANTS: [&str; 16] = [
    "INV-001", "INV-002", "INV-003", "INV-004", "INV-005", "INV-006", "INV-007", "INV-008",
    "INV-009", "INV-010", "INV-011", "INV-012", "INV-013", "INV-014", "INV-015", "INV-016",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContractError {
    Json(String),
    MissingIdentity,
    UnsupportedSchema,
    InvalidPath(String),
    DuplicateEntry,
    UnknownInvariant(String),
    Mismatch,
    OutsidePolicy,
    MissingRequiredInvariant,
}

/// Immutable contract result, not a permit. Accessors expose shared borrows only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedContract {
    task: TaskDocument,
}

/// Parse and check the policy declaration. Does not authenticate its owner.
pub fn parse_policy(text: &str) -> Result<PolicyDocument, ContractError> {
    let policy = decode_policy(text)?;
    check_policy(&policy)?;
    Ok(policy)
}

/// Parse and check the task declaration. Does not authenticate its approval.
pub fn parse_task(text: &str) -> Result<TaskDocument, ContractError> {
    let task = decode_task(text)?;
    check_task(&task)?;
    Ok(task)
}

/// Compare a request with independently trusted policy and approved scope.
/// The caller must load both trusted inputs outside model-controlled channels.
/// Supplying three caller-invented documents does not establish human authority.
pub fn validate_contract(
    policy: &PolicyDocument,
    requested: &TaskDocument,
    approved: &TaskDocument,
) -> Result<ValidatedContract, ContractError> {
    check_policy(policy)?;
    check_task(requested)?;
    check_task(approved)?;
    check_binding(policy, requested)?;
    check_approval(requested, approved)?;
    check_ceiling(policy, requested)?;
    Ok(canonical_contract(requested))
}

impl ValidatedContract {
    pub fn task(&self) -> &TaskDocument {
        &self.task
    }
}

fn decode_policy(text: &str) -> Result<PolicyDocument, ContractError> {
    json::decode(text).map_err(ContractError::Json)
}

fn decode_task(text: &str) -> Result<TaskDocument, ContractError> {
    json::decode(text).map_err(ContractError::Json)
}

fn check_policy(policy: &PolicyDocument) -> Result<(), ContractError> {
    check_header(policy.schema_version, &policy.repository_id)?;
    check_reference(&policy.policy)?;
    check_paths(&policy.paths)?;
    check_capabilities(&policy.capabilities, &policy.paths)?;
    check_invariants(&policy.invariants)
}

fn check_task(task: &TaskDocument) -> Result<(), ContractError> {
    check_header(task.schema_version, &task.repository_id)?;
    check_text(&task.scope.task_id)?;
    check_text(&task.scope.approval_reference)?;
    check_reference(&task.scope.policy)?;
    check_paths(&task.scope.paths)?;
    check_capabilities(&task.scope.capabilities, &task.scope.paths)?;
    check_invariants(&task.scope.invariants)
}

fn check_binding(policy: &PolicyDocument, task: &TaskDocument) -> Result<(), ContractError> {
    if policy.repository_id != task.repository_id {
        return Err(ContractError::Mismatch);
    }
    if policy.policy != task.scope.policy {
        return Err(ContractError::Mismatch);
    }
    Ok(())
}

fn check_approval(task: &TaskDocument, approved: &TaskDocument) -> Result<(), ContractError> {
    if canonical_task(task) != canonical_task(approved) {
        return Err(ContractError::Mismatch);
    }
    Ok(())
}

fn check_ceiling(policy: &PolicyDocument, task: &TaskDocument) -> Result<(), ContractError> {
    check_subset(&task.scope.paths.read, &policy.paths.read)?;
    check_subset(&task.scope.paths.write, &policy.paths.write)?;
    check_subset(&task.scope.paths.create, &policy.paths.create)?;
    check_subset(&task.scope.capabilities, &policy.capabilities)?;
    if !is_subset(&policy.invariants, &task.scope.invariants) {
        return Err(ContractError::MissingRequiredInvariant);
    }
    Ok(())
}

fn check_header(version: u32, repository: &str) -> Result<(), ContractError> {
    if version != SCHEMA_VERSION {
        return Err(ContractError::UnsupportedSchema);
    }
    check_text(repository)
}

fn check_reference(reference: &PolicyReference) -> Result<(), ContractError> {
    check_text(&reference.owner)?;
    check_text(&reference.version)?;
    check_text(&reference.reference)
}

fn check_paths(paths: &PathScope) -> Result<(), ContractError> {
    check_path_list(&paths.read)?;
    check_path_list(&paths.write)?;
    check_path_list(&paths.create)
}

fn check_path_list(paths: &[String]) -> Result<(), ContractError> {
    check_unique(paths)?;
    for path in paths {
        check_path(path)?;
    }
    Ok(())
}

fn check_capabilities(capabilities: &[Capability], paths: &PathScope) -> Result<(), ContractError> {
    check_unique(capabilities)?;
    for capability in capabilities {
        check_capability(capability, paths)?;
    }
    Ok(())
}

fn check_capability(capability: &Capability, paths: &PathScope) -> Result<(), ContractError> {
    check_path(capability_target(capability))?;
    if !capability_in_paths(capability, paths) {
        return Err(ContractError::OutsidePolicy);
    }
    Ok(())
}

fn check_invariants(invariants: &[String]) -> Result<(), ContractError> {
    check_unique(invariants)?;
    for identifier in invariants {
        if !KNOWN_INVARIANTS.contains(&identifier.as_str()) {
            return Err(ContractError::UnknownInvariant(identifier.clone()));
        }
    }
    Ok(())
}

fn check_text(value: &str) -> Result<(), ContractError> {
    if is_blank(value) {
        return Err(ContractError::MissingIdentity);
    }
    Ok(())
}

fn check_path(path: &str) -> Result<(), ContractError> {
    if invalid_path_syntax(path) {
        return Err(ContractError::InvalidPath(path.to_owned()));
    }
    Ok(())
}

fn check_unique<T: Ord + Clone>(items: &[T]) -> Result<(), ContractError> {
    if has_duplicates(items) {
        return Err(ContractError::DuplicateEntry);
    }
    Ok(())
}

fn check_subset<T: PartialEq>(requested: &[T], ceiling: &[T]) -> Result<(), ContractError> {
    if !is_subset(requested, ceiling) {
        return Err(ContractError::OutsidePolicy);
    }
    Ok(())
}

fn canonical_contract(task: &TaskDocument) -> ValidatedContract {
    ValidatedContract {
        task: canonical_task(task),
    }
}

fn canonical_task(task: &TaskDocument) -> TaskDocument {
    let mut result = task.clone();
    result.scope.paths.read.sort();
    result.scope.paths.write.sort();
    result.scope.paths.create.sort();
    result.scope.capabilities.sort();
    result.scope.invariants.sort();
    result
}

fn capability_target(capability: &Capability) -> &str {
    match capability {
        Capability::ReadPath(path) | Capability::WritePath(path) | Capability::CreatePath(path) => {
            path
        }
        Capability::ProtectedAction(grant) => &grant.target,
    }
}

fn capability_in_paths(capability: &Capability, paths: &PathScope) -> bool {
    match capability {
        Capability::ReadPath(path) => paths.read.contains(path),
        Capability::WritePath(path) => paths.write.contains(path),
        Capability::CreatePath(path) => paths.create.contains(path),
        Capability::ProtectedAction(_) => true,
    }
}

fn invalid_path_syntax(path: &str) -> bool {
    path.is_empty()
        || path.chars().any(forbidden_path_character)
        || path.split('/').any(invalid_component)
}

fn forbidden_path_character(character: char) -> bool {
    matches!(character, '\\' | '\0' | '*' | '?' | '[' | ']')
}

fn invalid_component(component: &str) -> bool {
    matches!(component, "" | "." | "..")
}

fn is_blank(value: &str) -> bool {
    value.trim().is_empty()
}

fn has_duplicates<T: Ord + Clone>(items: &[T]) -> bool {
    let mut ordered = items.to_vec();
    ordered.sort();
    ordered.windows(2).any(|pair| pair[0] == pair[1])
}

fn is_subset<T: PartialEq>(requested: &[T], ceiling: &[T]) -> bool {
    requested.iter().all(|item| ceiling.contains(item))
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for ContractError {}
