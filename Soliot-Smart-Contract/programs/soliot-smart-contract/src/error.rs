use anchor_lang::prelude::error_code;

#[error_code]
pub enum IotContractError {
    #[msg("Node Already Exists")]
    NodeAlreadyExists,
}