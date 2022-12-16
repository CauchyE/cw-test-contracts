use cosmwasm_std::{StdError, Coin};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient Funds")]
    InsufficientFunds {coins: Vec<Coin>},

    #[error("Invalid Pool Route: {reason:?}")]
    InvalidPoolRoute { reason: String },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}