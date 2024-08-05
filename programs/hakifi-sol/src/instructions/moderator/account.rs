use anchor_lang::prelude::*;
use crate::state::moderator::Moderator;

#[derive(Accounts)]
#[instruction(_bump : u8)]

pub struct ManagementMod<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub moderator_account: Account<'info, Moderator>,
    pub system_program: Program<'info, System>,
}