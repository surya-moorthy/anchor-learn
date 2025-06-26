
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + UserAccount::INIT_SPACE,
         seeds = [b"user",user.key().as_ref()],
        bump,
    )]
    pub create_account: Account<'info, UserAccount>,

    pub system_program: Program<'info , System>
}

pub fn create_user(ctx: Context<CreateUser>,name : String) -> Result<()> {
    *ctx.accounts.create_account = UserAccount {
        user : ctx.accounts.user.key(),
        bump: ctx.bumps.create_account,
        user_account_address: ctx.accounts.create_account.key(),
        name,
    };
    Ok(())
}