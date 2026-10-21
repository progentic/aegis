use aegis_authority::{Capability, PathScope, PolicyReference, ProtectedAction, TaskScope};

#[test]
fn path_capability_variants_remain_distinct() {
    let path = String::from("src/config.rs");
    let read = Capability::ReadPath(path.clone());
    let write = Capability::WritePath(path.clone());
    let create = Capability::CreatePath(path);
    assert_ne!(read, write);
    assert_ne!(write, create);
    assert_ne!(read, create);
}

#[test]
fn protected_capability_retains_its_target() {
    let capability = Capability::ProtectedAction {
        action: ProtectedAction::DependencyChange,
        target: String::from("Cargo.toml"),
    };
    let Capability::ProtectedAction { action, target } = capability else {
        panic!("protected action shape was lost");
    };
    assert_eq!(action, ProtectedAction::DependencyChange);
    assert_eq!(target, "Cargo.toml");
}

#[test]
fn scope_represents_explicit_empty_inputs() {
    let scope = empty_scope();
    assert!(scope.paths.read.is_empty());
    assert!(scope.paths.write.is_empty());
    assert!(scope.paths.create.is_empty());
    assert!(scope.capabilities.is_empty());
    assert!(scope.invariants.is_empty());
    assert_eq!(scope.task_id, "TASK-1");
    assert_eq!(scope.policy.version, "policy-1");
    assert_eq!(scope.approval_reference, "human-approval-1");
}

fn empty_scope() -> TaskScope {
    TaskScope {
        task_id: String::from("TASK-1"),
        approval_reference: String::from("human-approval-1"),
        policy: policy_reference(),
        paths: empty_paths(),
        capabilities: Vec::new(),
        invariants: Vec::new(),
    }
}

fn policy_reference() -> PolicyReference {
    PolicyReference {
        owner: String::from("human-owner"),
        version: String::from("policy-1"),
        reference: String::from("policy-record-1"),
    }
}

fn empty_paths() -> PathScope {
    PathScope {
        read: Vec::new(),
        write: Vec::new(),
        create: Vec::new(),
    }
}
