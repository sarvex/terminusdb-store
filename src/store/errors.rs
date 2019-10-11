//! Error types for store
use failure::{Fail,Error};
use crate::storage::LabelCreationError;

/// Possible errors for `Store::create()`.
#[derive(Debug, Fail)]
pub enum NamedGraphCreationError {
    #[fail(display = "named graph already exists")]
    AlreadyExists,
    #[fail(display = "unexpected error: {}", _0)]
    Unexpected(#[fail(cause)] Error)
}

impl From<LabelCreationError> for NamedGraphCreationError {
    fn from(e: LabelCreationError) -> NamedGraphCreationError {
        match e {
            LabelCreationError::AlreadyExists => Self::AlreadyExists,
            LabelCreationError::Unexpected(e) => Self::Unexpected(e)
        }
    }
}
