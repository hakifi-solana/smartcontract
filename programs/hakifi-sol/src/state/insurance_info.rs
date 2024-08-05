use solana_program::pubkey::Pubkey;
use anchor_lang::prelude::*;

use crate::enums::State;

#[account]
#[derive(Default)]
pub struct InsuranceInfor {
    pub id: String,
    pub buyer: Pubkey,
    pub margin: u64,
    pub claim_amount: u64,
    pub expired_time: u64,
    pub open_time: u64,
    pub state: State,
    pub valid: bool,
}

impl InsuranceInfor {
    pub const LEN: usize = 32 + 32 + 8*4+ 1 + 1;
}
