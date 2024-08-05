use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

use super::account::ProcessInsurance;
use crate::enums::State;
use crate::enums::Type;
use crate::errors::ErrorCode;
use crate::events::insurance::EInsurance;
use crate::constants::Config;

pub fn refund_insurance(ctx: Context<ProcessInsurance>, _id: String) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    let insurance_account = &mut ctx.accounts.insurance_account;
    insurance_account.state = State::REFUNDED;

    ctx.accounts.vault_account.margin_pool -= insurance_account.margin;

    let bump_vector = &ctx.accounts.vault_account.bump.to_le_bytes();

    let inner = vec![Config::default().vault_pda_seed.as_ref(), bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];
    let transfer_instruction = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.creator_token_account.to_account_info(),
        authority: ctx.accounts.vault_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );

    anchor_spl::token::transfer(cpi_ctx, insurance_account.margin)?;

    insurance_account.margin = 0;
    emit!(EInsurance {
        id_insurance: insurance_account.id.clone(),
        buyer: insurance_account.buyer,
        margin: 0,
        claim_amount: insurance_account.claim_amount,
        expired_time: insurance_account.expired_time,
        open_time: insurance_account.open_time,
        state: State::REFUNDED,
        event_type: Type::REFUND,
    });

    Ok(())
}

pub fn cancel_insurance(ctx: Context<ProcessInsurance>, _id: String) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    let insurance_account = &mut ctx.accounts.insurance_account;
    insurance_account.state = State::CANCELED;
    ctx.accounts.vault_account.margin_pool -= insurance_account.margin;

    let bump_vector = &ctx.accounts.vault_account.bump.to_le_bytes();

    let inner = vec![Config::default().vault_pda_seed.as_ref(), bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];
    let transfer_instruction = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.creator_token_account.to_account_info(),
        authority: ctx.accounts.vault_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );

    anchor_spl::token::transfer(cpi_ctx, insurance_account.margin)?;

    insurance_account.margin = 0;

    emit!(EInsurance {
        id_insurance: insurance_account.id.clone(),
        buyer: insurance_account.buyer,
        margin: 0,
        claim_amount: insurance_account.claim_amount,
        expired_time: insurance_account.expired_time,
        open_time: insurance_account.open_time,
        state: State::CANCELED,
        event_type: Type::CANCEL,
    });

    Ok(())
}

pub fn claim_insurance(ctx: Context<ProcessInsurance>, _id: String) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    let insurance_account = &mut ctx.accounts.insurance_account;
    insurance_account.state = State::CLAIMED;
    ctx.accounts.vault_account.claim_pool -= insurance_account.claim_amount;

    let bump_vector = &ctx.accounts.vault_account.bump.to_le_bytes();

    let inner = vec![Config::default().vault_pda_seed.as_ref(), bump_vector.as_ref()];
    let outer = vec![inner.as_slice()];
    let transfer_instruction = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.creator_token_account.to_account_info(),
        authority: ctx.accounts.vault_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );

    anchor_spl::token::transfer(cpi_ctx, insurance_account.claim_amount)?;

    insurance_account.claim_amount = 0;

    emit!(EInsurance {
        id_insurance: insurance_account.id.clone(),
        buyer: insurance_account.buyer,
        margin: 0,
        claim_amount: 0,
        expired_time: insurance_account.expired_time,
        open_time: insurance_account.open_time,
        state: State::CLAIMED,
        event_type: Type::CLAIM,
    });

    Ok(())
}