 mod constants;
 mod errors;
 mod instructions;
 mod state;
 mod enums;
 mod events;

use anchor_lang::prelude::*;
declare_id!("8K3zrnRzy9sAo7qr1gPrGixEYsaixkn6ZdyBgMN4hnMm");

use instructions::initialize::account::*;
use instructions::create_insurance::account::*;
use instructions::delete_insurance::account::*;
use instructions::expire_liquidate::account::*;
use instructions::moderator::account::*;
use instructions::process_insurance::account::*;
use instructions::update_available_insurance::account::*;
use instructions::update_invalid_insurance::account::*;

use borsh::BorshDeserialize;

#[program]
pub mod insurance {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _bump: u8) -> Result<()> {
        instructions::initialize::instruction::initialize(
            ctx,
            _bump
        )
    }

    pub fn initializetokenpda(ctx: Context<InitializeVaultTokenPda>) -> Result<()> {
        instructions::initialize::instruction::initializetokenpda(
            ctx
        )
    }

    pub fn add_mod(ctx: Context<ManagementMod>, _mod_address: Pubkey) -> Result<()> {
        instructions::moderator::instruction::add_mod(
            ctx,
            _mod_address
        )
    }

    pub fn delete_mod(ctx: Context<ManagementMod>, _mod_address: Pubkey) -> Result<()> {
        instructions::moderator::instruction::delete_mod(
            ctx,
            _mod_address
        )
    }

    pub fn create_insurance(ctx: Context<CreateInsurance>, id: String, margin: u64) -> Result<()> {
        instructions::create_insurance::instruction::create_insurance(
            ctx,
            id,
            margin
        )
    }

    pub fn update_available_insurance(
        ctx: Context<UpdateAvailableInsurance>,
        _id: String,
        claim_amount: u64,
        expired_time: u64,
    ) -> Result<()> {
        instructions::update_available_insurance::instruction::update_available_insurance(
            ctx,
            _id,
            claim_amount,
            expired_time,
        )
    }

    pub fn update_invalid_insurance(
        ctx: Context<UpdateInvalidInsurance>,
        _id: String,
    ) -> Result<()> {
        instructions::update_invalid_insurance::instruction::update_invalid_insurance(
            ctx,
            _id
        )
    }

    pub fn refund_insurance(ctx: Context<ProcessInsurance>, _id: String) -> Result<()> {
        instructions::process_insurance::instruction::refund_insurance(
            ctx,
            _id
        )
    }

    pub fn cancel_insurance(ctx: Context<ProcessInsurance>, _id: String) -> Result<()> {
        instructions::process_insurance::instruction::cancel_insurance(
            ctx,
            _id
        )
    }

    pub fn claim_insurance(ctx: Context<ProcessInsurance>, _id: String) -> Result<()> {
        instructions::process_insurance::instruction::claim_insurance(
            ctx,
            _id
        )
    }
    pub fn expire_insurance(ctx: Context<ExpireLiquidateInsurance>, _id: String) -> Result<()> {
        instructions::expire_liquidate::instruction::expire_insurance(
            ctx,
            _id
        )
    }

    pub fn liquidate_insurance(ctx: Context<ExpireLiquidateInsurance>, _id: String) -> Result<()> {
        instructions::expire_liquidate::instruction::liquidate_insurance(
            ctx,
            _id
        )
    }

    pub fn delete_insurance(ctx: Context<DeleteInsurance>, _id: String) -> Result<()> {
        instructions::delete_insurance::instruction::delete_insurance(
            ctx,
            _id
        )
    }
}
