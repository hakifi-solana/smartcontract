use anchor_lang::prelude::*;

use super::account::ExpireLiquidateInsurance;
use crate::enums::State;
use crate::enums::Type;
use crate::errors::ErrorCode;
use crate::events::insurance::EInsurance;

pub fn expire_insurance(ctx: Context<ExpireLiquidateInsurance>, _id: String) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    let insurance_account = &mut ctx.accounts.insurance_account;
    insurance_account.state = State::EXPIRED;
    emit!(EInsurance {
        id_insurance: insurance_account.id.clone(),
        buyer: insurance_account.buyer,
        margin: insurance_account.margin,
        claim_amount: insurance_account.claim_amount,
        expired_time: insurance_account.expired_time,
        open_time: insurance_account.open_time,
        state: State::EXPIRED,
        event_type: Type::EXPIRED,
    });

    Ok(())
}

pub fn liquidate_insurance(ctx: Context<ExpireLiquidateInsurance>, _id: String) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    let insurance_account = &mut ctx.accounts.insurance_account;
    insurance_account.state = State::LIQUIDATED;
    emit!(EInsurance {
        id_insurance: insurance_account.id.clone(),
        buyer: insurance_account.buyer,
        margin: insurance_account.margin,
        claim_amount: insurance_account.claim_amount,
        expired_time: insurance_account.expired_time,
        open_time: insurance_account.open_time,
        state: State::LIQUIDATED,
        event_type: Type::LIQUIDATED,
    });

    Ok(())
}