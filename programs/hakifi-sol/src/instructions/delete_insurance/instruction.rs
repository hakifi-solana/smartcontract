use anchor_lang::prelude::*;

use super::account::DeleteInsurance;
use crate::errors::ErrorCode;

pub fn delete_insurance(ctx: Context<DeleteInsurance>, _id: String) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator
            .moderator_list
            .contains(&ctx.accounts.authority.key()),
        ErrorCode::NotPerMission
    );
    Ok(())
}