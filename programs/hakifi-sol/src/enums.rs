use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

#[derive(Copy, Clone, BorshSerialize, BorshDeserialize)]
pub enum State {
    PENDING,
    AVAILABLE,
    CLAIMED,
    REFUNDED,
    LIQUIDATED,
    EXPIRED,
    CANCELED,
    INVALID,
}

impl Default for State {
    fn default() -> Self {
        State::PENDING
    }
}

#[derive(Copy, Clone, BorshSerialize, BorshDeserialize)]
pub enum Type {
    CREATE,
    UPDATEAVAILABLE,
    UPDATEINVALID,
    REFUND,
    CANCEL,
    CLAIM,
    EXPIRED,
    LIQUIDATED,
}

impl Default for Type {
    fn default() -> Self {
        Type::CREATE
    }
}