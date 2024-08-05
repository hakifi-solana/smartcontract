use anchor_lang::prelude::*;
use crate::constants::MAX_MODERATOR_COUNT;

#[account]
pub struct Moderator {
    pub moderator_list: Vec<Pubkey>,
}

impl Moderator {
    pub const LEN : usize  = 32 * MAX_MODERATOR_COUNT;
}