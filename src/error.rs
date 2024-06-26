use cosmwasm_std::{Addr, StdError};
use nibiru_std::errors::NibiruError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("{0}")]
    NibiruError(#[from] NibiruError),
}