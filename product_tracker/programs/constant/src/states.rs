use anchor_lang::prelude::*;

#[account]
#[derive(Default)]

pub struct UserAccount {
    pub address: Pubkey, // 32
    pub certificate: String, // 4 + 2048
    pub role: String, // 4 + 256
    pub product_id: u8, // 1
}

#[account]
pub struct ProductAccount {
    pub record: Vec<Record>,  // 
    pub record_count: u8,  // 1
    pub product_name: String,  // 4 + 256
    pub product_origin: Pubkey,  // 32
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Record {
    pub location: String,  // 4 + 256
    pub next_owner: Pubkey,  // 32
    pub certificate: String, // 4 + 2048
    pub role: String, // 4 + 256
}