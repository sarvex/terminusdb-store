//! Error types for storage
use failure::{Fail,Error};

/// Possible errors for `Store::create()`.
#[derive(Debug, Fail)]
pub enum LabelCreationError {
    #[fail(display = "label already exists")]
    AlreadyExists,
    #[fail(display = "unexpected error: {}", _0)]
    Unexpected(#[fail(cause)] Error)
}
