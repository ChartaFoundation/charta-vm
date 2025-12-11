use thiserror::Error;

pub type Result<T> = std::result::Result<T, VMError>;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("IR loading error: {0}")]
    IrLoad(#[from] charta_core::error::ValidationError),
    
    #[error("Signal/coil not found: {0}")]
    NotFound(String),
    
    #[error("Invalid guard expression: {0}")]
    InvalidGuard(String),
    
    #[error("VM execution error: {0}")]
    Execution(String),
}
