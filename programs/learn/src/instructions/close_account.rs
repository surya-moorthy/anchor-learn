
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosUser<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mut,
        seeds = [b"user",user.key().as_ref()],
        bump,
        close = user
    )]
    pub user_account: Account<'info , UserAccount>,
}

pub fn close_user(_ctx : Context<ClosUser>) -> Result<()>  {   
    Ok(())
}