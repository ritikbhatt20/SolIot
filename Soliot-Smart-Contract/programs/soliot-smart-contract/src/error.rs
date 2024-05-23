use anchor_lang::prelude::error_code;

#[error_code]
pub enum IotError {
    #[msg("Node has already updated in this block")]
    UpdateAlreadyCalled,
}