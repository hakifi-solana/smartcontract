use anchor_lang::prelude::*;
use crate::enums::{State, Type};
#[event]
pub struct EInsurance {
    pub id_insurance: String,
    pub buyer: Pubkey,
    pub margin: u64,
    pub claim_amount: u64,
    pub expired_time: u64,
    pub open_time: u64,
    pub state: State,
    pub event_type: Type,
}