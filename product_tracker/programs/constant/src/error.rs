use anchor_lang::prelude::*;

#[error_code]
pub enum TrackerError {
    #[msg("You are not authorized to add record to this product.")]
    Unauthorized,
    #[msg("There are too many records.")]
    Overflowed,
}