use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::vault_info::Vault;
use crate::state::moderator::Moderator;
use crate::constants::Config;
 
#[derive(Accounts)]
#[instruction(_bump : u8)]

pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Vault::LEN + 8,
        seeds=[Config::default(). vault_pda_seed.as_ref()],
        bump
    )]
    pub vault_account: Account<'info, Vault>,

    #[account(
        init,
        payer = owner,
        space = 8 + Moderator::LEN,
        seeds=[Config::default().moderator_pda_seed.as_ref()],
        bump
    )]
    pub moderator_account: Account<'info, Moderator>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeVaultTokenPda<'info> {
    #[account(
        init,
        seeds = [owner.key.as_ref()],
        bump,
        payer = owner,
        token::mint = mint,
        token::authority = vault_pda,
     )]
    // vault token account
    pub vault_token_account: Account<'info, TokenAccount>,
    /// vault pda
    pub vault_pda: Account<'info, Vault>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}