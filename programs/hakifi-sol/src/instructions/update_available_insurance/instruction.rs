use anchor_lang::prelude::*;

use super::account::UpdateAvailableInsurance;
use crate::enums::State;
use crate::enums::Type;
use crate::errors::ErrorCode;
use crate::events::insurance::EInsurance;

pub fn update_available_insurance(
    ctx: Context<UpdateAvailableInsurance>,
    _id: String,
    claim_amount: u64,
    expired_time: u64,
) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    let insurance_account = &mut ctx.accounts.insurance_account;
    insurance_account.state = State::AVAILABLE;
    insurance_account.claim_amount = claim_amount;
    insurance_account.expired_time = expired_time;

    ctx.accounts.vault_account.claim_pool += claim_amount;
    emit!(EInsurance {
        id_insurance: insurance_account.id.clone(),
        buyer: insurance_account.buyer,
        margin: insurance_account.margin,
        claim_amount: claim_amount,
        expired_time: expired_time,
        open_time: insurance_account.open_time,
        state: State::AVAILABLE,
        event_type: Type::UPDATEAVAILABLE,
    });
    msg!("Update available successfully");

    Ok(())
}