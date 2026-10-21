//! Data-only authority inputs. Constructing a value does not grant permission.
//!
//! Every field is unvalidated, including ownership and approval references.
//! There is no policy loader, path matcher, decision engine, or executor here.
//! Serialization, capability semantics, and validation require later decisions.

/// A declared policy reference, not verified provenance or ownership.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PolicyReference {
    pub owner: String,
    pub version: String,
    pub reference: String,
}

/// A task's declared inputs, not an authenticated or validated authority set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskScope {
    pub task_id: String,
    pub approval_reference: String,
    pub policy: PolicyReference,
    pub paths: PathScope,
    pub capabilities: Vec<Capability>,
    pub invariants: Vec<String>,
}

/// Separate path lists. Entries are opaque; no normalization or glob rules exist.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathScope {
    pub read: Vec<String>,
    pub write: Vec<String>,
    pub create: Vec<String>,
}

/// Distinct requested capability shapes, not issued permits or executable rights.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Capability {
    ReadPath(String),
    WritePath(String),
    CreatePath(String),
    ProtectedAction {
        action: ProtectedAction,
        target: String,
    },
}

/// Protected action categories already identified by repository governance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtectedAction {
    NewModule,
    PublicInterfaceChange,
    DependencyChange,
    SchemaChange,
    ServiceConnection,
    PermissionChange,
    ProtectedPathChange,
}
