//! Debug output must not copy authority documents into logs.

use crate::{
    Capability, PathScope, PolicyDocument, PolicyReference, ProtectedGrant, TaskDocument, TaskScope,
};
use std::fmt;

impl fmt::Debug for PolicyDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyDocument")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for TaskDocument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskDocument")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for PolicyReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolicyReference")
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for TaskScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("TaskScope").finish_non_exhaustive()
    }
}

impl fmt::Debug for PathScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("PathScope").finish_non_exhaustive()
    }
}

impl fmt::Debug for Capability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Capability(..)")
    }
}

impl fmt::Debug for ProtectedGrant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProtectedGrant")
            .finish_non_exhaustive()
    }
}
