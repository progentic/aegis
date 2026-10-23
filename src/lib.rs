//! Pure authority-contract validation. No filesystem or execution API exists.
//!
//! Every field is unvalidated, including ownership and approval references.
//! There is no policy loader, path matcher, decision engine, or executor here.
//! Use the contract API to check inputs against independently trusted policy
//! and approval data. Successful validation is not authentication or a permit.

mod contract;
mod diagnostics;
mod error;
mod json;
pub mod limits;

pub use contract::{ValidatedContract, parse_policy, parse_task, validate_contract};
pub use error::ContractError;
use serde::{Deserialize, Serialize};

/// Versioned policy ceiling. Its provenance must be established by the caller.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyDocument {
    pub schema_version: u32,
    pub repository_id: String,
    #[serde(deserialize_with = "json::object")]
    pub policy: PolicyReference,
    #[serde(deserialize_with = "json::object")]
    pub paths: PathScope,
    pub capabilities: Vec<Capability>,
    pub invariants: Vec<String>,
}

/// Versioned task declaration, not an authenticated human approval.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskDocument {
    pub schema_version: u32,
    pub repository_id: String,
    #[serde(deserialize_with = "json::object")]
    pub scope: TaskScope,
}

/// A declared policy reference, not verified provenance or ownership.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyReference {
    pub owner: String,
    pub version: String,
    pub reference: String,
}

/// A task's declared inputs, not an authenticated or validated authority set.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskScope {
    pub task_id: String,
    pub approval_reference: String,
    #[serde(deserialize_with = "json::object")]
    pub policy: PolicyReference,
    #[serde(deserialize_with = "json::object")]
    pub paths: PathScope,
    pub capabilities: Vec<Capability>,
    pub invariants: Vec<String>,
}

/// Separate path lists. Entries are opaque; no normalization or glob rules exist.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathScope {
    pub read: Vec<String>,
    pub write: Vec<String>,
    pub create: Vec<String>,
}

/// Distinct declared capabilities, not issued permits or executable rights.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Capability {
    ReadPath(String),
    WritePath(String),
    CreatePath(String),
    ProtectedAction(#[serde(deserialize_with = "json::object")] ProtectedGrant),
}

/// An exact-target protected-action declaration.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedGrant {
    #[serde(deserialize_with = "json::string_enum")]
    pub action: ProtectedAction,
    pub target: String,
}

/// Protected action categories already identified by repository governance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProtectedAction {
    NewModule,
    PublicInterfaceChange,
    DependencyChange,
    SchemaChange,
    ServiceConnection,
    PermissionChange,
    ProtectedPathChange,
}
