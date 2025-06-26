use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub user: Pubkey,
    pub bump : u8,
    #[max_len(32)]
    pub name: String,
    pub user_account_address : Pubkey
}