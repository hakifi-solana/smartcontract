use anchor_lang::prelude::*;
use super::account::Initialize;
use super::account::InitializeVaultTokenPda;

pub fn initialize(ctx: Context<Initialize>, _bump: u8) -> Result<()>{
    let vault = &mut ctx.accounts.vault_account;
    let moderator = &mut ctx.accounts.moderator_account;
    moderator.moderator_list.push(ctx.accounts.owner.key());
    vault.bump = _bump;
    vault.margin_pool = 0;
    vault.claim_pool = 0;
    vault.hakifi_fund = 0;
    vault.third_party_fund = 0;
    Ok(())
}

pub fn initializetokenpda(ctx: Context<InitializeVaultTokenPda>) -> Result<()>{
    let pda = ctx.accounts.vault_token_account.key();
    msg!("Token account vault pda : {}", pda);
    Ok(())
}