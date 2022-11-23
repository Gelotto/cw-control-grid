use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("CellNotFound")]
  CellNotFound {},

  #[error("TooManyTickets")]
  TooManyTickets {},

  #[error("InsufficientFunds")]
  InsufficientFunds {},

  #[error("ExcessiveFunds")]
  ExcessiveFunds {},
}
