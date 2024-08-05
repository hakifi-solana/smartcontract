use anchor_lang::prelude::*;
use crate::state::moderator::Moderator;
use crate::state::insurance_info::InsuranceInfor;
use crate::constants::Config;
#[derive(Accounts)]
#[instruction(id : String)]
pub struct DeleteInsurance<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [Config::default().insurance_pda_seed.as_ref(), id.as_bytes()], 
        bump
    )]
    pub insurance_account: Account<'info, InsuranceInfor>,
    #[account(mut)]
    pub moderator_account: Account<'info, Moderator>,
    pub system_program: Program<'info, System>,
}