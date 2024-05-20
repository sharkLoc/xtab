use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Xerror {
    #[error("stdin not detected")]
    StdinNotDetected,

    #[error("failed to open file: {0}")]
    IoError(#[from] io::Error),
}
