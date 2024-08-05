pub const MAX_MODERATOR_COUNT:usize = 20;

pub struct Config {
    pub vault_pda_seed: &'static str,
    pub insurance_pda_seed: &'static str,
    pub moderator_pda_seed: &'static str,
    pub vault_token_account_seed: &'static str,
    pub moderator_len: usize, 
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vault_pda_seed: "vault",
            insurance_pda_seed: "insurance",
            moderator_pda_seed: "moderator",
            vault_token_account_seed: "vault token account",
            moderator_len: 20,
        }
    }
}

