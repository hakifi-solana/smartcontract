use anchor_lang::prelude::*;
use super::account::ManagementMod;
use crate::errors::ErrorCode;

pub fn add_mod(ctx: Context<ManagementMod>, _mod_address: Pubkey) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator.moderator_list.contains(&ctx.accounts.owner.key()),
        ErrorCode::NotPerMission
    );
    require!(
        !moderator.moderator_list.contains(&_mod_address),
        ErrorCode::ModAlreadyExists
    );
    moderator.moderator_list.push(_mod_address);
    Ok(())
}

pub fn delete_mod(ctx: Context<ManagementMod>, _mod_address: Pubkey) -> Result<()> {
    let moderator = &mut ctx.accounts.moderator_account;
    require!(
        moderator.moderator_list.contains(&ctx.accounts.owner.key()),
        ErrorCode::NotPerMission
    );
    require!(
        moderator.moderator_list.contains(&_mod_address),
        ErrorCode::ModNotExists
    );
    let mut new_moderators = moderator.moderator_list.clone();
    new_moderators.retain(|&key| key != _mod_address);
    moderator.moderator_list = new_moderators;
    Ok(())
}