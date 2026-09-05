use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Subprocess returns with non-zero status: {status}")]
    NonZeroExitStatus { status: i32 },
    #[error("Subprocess cannot start: {error:?}")]
    SubprocessCannotStart { error: io::Error },
    #[error(transparent)]
    Var(#[from] std::env::VarError),
}
