//! Technical limits applied before parsing, copying, or sorting authority data.

/// Maximum UTF-8 document size, including whitespace.
pub const MAX_DOCUMENT_BYTES: usize = 256 * 1024;
/// Maximum number of entries in any authority set.
pub const MAX_SET_ENTRIES: usize = 1024;
/// Maximum UTF-8 size of an identity or reference.
pub const MAX_IDENTITY_BYTES: usize = 256;
/// Maximum UTF-8 repository-relative path size.
pub const MAX_PATH_BYTES: usize = 4095;
/// Maximum UTF-8 size of a single path component on the selected Linux profile.
pub const MAX_COMPONENT_BYTES: usize = 255;

pub(crate) fn oversized_document(text: &str) -> bool {
    text.len() > MAX_DOCUMENT_BYTES
}

pub(crate) fn oversized_set<T>(items: &[T]) -> bool {
    items.len() > MAX_SET_ENTRIES
}

pub(crate) fn oversized_identity(value: &str) -> bool {
    value.len() > MAX_IDENTITY_BYTES
}

pub(crate) fn oversized_path(value: &str) -> bool {
    value.len() > MAX_PATH_BYTES
}

pub(crate) fn oversized_component(value: &str) -> bool {
    value.len() > MAX_COMPONENT_BYTES
}
