use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::state::vault_info::Vault;
use crate::state::insurance_info::InsuranceInfor;
use crate::constants::Config;
#[derive(Accounts)]
#[instruction(id : String)]
pub struct CreateInsurance<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = InsuranceInfor::LEN + 8,
        seeds = [Config::default().insurance_pda_seed.as_ref(), id.as_bytes()], 
        bump
    )]
    pub insurance_account: Account<'info, InsuranceInfor>,

    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_account: Account<'info, Vault>,

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}