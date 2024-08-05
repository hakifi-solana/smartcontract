use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::Transfer;

use super::account::CreateInsurance;
use crate::enums::State;
use crate::enums::Type;
use crate::errors::ErrorCode;
use crate::events::insurance::EInsurance;
use crate::constants::Config;

pub fn create_insurance(ctx: Context<CreateInsurance>, id: String, margin: u64) -> Result<()> {
    let insurance_account = &mut ctx.accounts.insurance_account;
    require!(
        ctx.accounts.creator_token_account.amount > margin,
        ErrorCode::InvalidInsufficientBallance
    );
    insurance_account.id = id.clone();
    insurance_account.buyer = *ctx.accounts.authority.key;
    insurance_account.margin = margin;
    insurance_account.claim_amount = 0;
    insurance_account.expired_time = 0;
    insurance_account.open_time = clock::Clock::get()?.unix_timestamp.try_into().unwrap();
    insurance_account.state = State::PENDING;
    insurance_account.valid = true;
    let authority = &ctx.accounts.authority;
    ctx.accounts.vault_account.margin_pool += margin;

    // Transfer tokens from taker to initializer
    let bump_vector = &ctx.accounts.vault_account.bump.to_le_bytes();
    let inner = vec![Config::default().vault_pda_seed.as_ref(),bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];

    let transfer_instruction = Transfer {
        from: ctx.accounts.creator_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: authority.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );
    anchor_spl::token::transfer(cpi_ctx, margin)?;

    emit!(EInsurance {
        id_insurance: id,
        buyer: *ctx.accounts.authority.key,
        margin,
        claim_amount: 0,
        expired_time: 0,
        open_time: clock::Clock::get()?.unix_timestamp.try_into().unwrap(),
        state: State::PENDING,
        event_type: Type::CREATE,
    });

    msg!("Create insurance successfully");
    Ok(())
}
