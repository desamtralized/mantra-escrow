use cosmwasm_std::StdError;
use thiserror::Error;

use crate::state::EscrowState;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Invalid config: {msg}")]
    InvalidConfig { msg: String },
    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Invalid funds")]
    InvalidFunds {},
    #[error("Escrow not found")]
    EscrowNotFound {},
    #[error("Escrow timeout")]
    EscrowTimeout {},
    #[error("Invalid escrow state, expected {expected:?}, got {got:?}")]
    InvalidEscrowState {
        expected: EscrowState,
        got: EscrowState,
    },
    #[error("Invalid address")]
    InvalidAddress {},
}

impl From<ContractError> for StdError {
    fn from(e: ContractError) -> Self {
        match e {
            ContractError::Std(e) => e,
            _ => StdError::generic_err(e.to_string()),
        }
    }
}
