use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub bump: u8,
    pub margin_pool: u64,
    pub claim_pool: u64,
    pub hakifi_fund: u64,
    pub third_party_fund: u64,
}

impl Vault {
    pub const LEN : usize  = 1 + 8 * 4;
}