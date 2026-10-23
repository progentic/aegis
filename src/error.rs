//! Stable diagnostics contain no caller-supplied values or parser excerpts.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContractError {
    Json,
    DocumentTooLarge,
    TooManyEntries,
    IdentityTooLong,
    MissingIdentity,
    InvalidIdentity,
    UnsupportedSchema,
    InvalidPath,
    DuplicateEntry,
    UnknownInvariant,
    Mismatch,
    OutsidePolicy,
    MissingRequiredInvariant,
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl ContractError {
    pub fn code(self) -> &'static str {
        match self {
            Self::Json => "CONTRACT_INVALID_JSON",
            Self::DocumentTooLarge => "CONTRACT_DOCUMENT_TOO_LARGE",
            Self::TooManyEntries => "CONTRACT_TOO_MANY_ENTRIES",
            Self::IdentityTooLong => "CONTRACT_IDENTITY_TOO_LONG",
            Self::MissingIdentity => "CONTRACT_MISSING_IDENTITY",
            Self::InvalidIdentity => "CONTRACT_INVALID_IDENTITY",
            Self::UnsupportedSchema => "CONTRACT_UNSUPPORTED_SCHEMA",
            Self::InvalidPath => "CONTRACT_INVALID_PATH",
            Self::DuplicateEntry => "CONTRACT_DUPLICATE_ENTRY",
            Self::UnknownInvariant => "CONTRACT_UNKNOWN_INVARIANT",
            Self::Mismatch => "CONTRACT_MISMATCH",
            Self::OutsidePolicy => "CONTRACT_OUTSIDE_POLICY",
            Self::MissingRequiredInvariant => "CONTRACT_MISSING_REQUIRED_INVARIANT",
        }
    }
}

impl std::error::Error for ContractError {}
