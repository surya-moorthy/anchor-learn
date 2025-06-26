use anchor_lang::prelude::*;

declare_id!("CAVZT8xhXfRKhZSBEJ884dvjpDsTaDZjjT3MxXNumbwj");

#[program]
pub mod learn {
    use anchor_lang::system_program::{CreateAccount,create_account};

    use crate::instruction::HelloWorld;

    use super::*;
  
    pub fn hello_world(_ctx : Context<Hello> )->Result<()> {
        msg!("Hello, World!");

        msg!("this is a basic program which has a programid : {}",&id());
        Ok(())
    } 

    pub fn create_system_account(ctx : Context<CreateSystemAccount>) -> Result<()> {
        msg!("System program is invoked, lets create an account");

        msg!("The new account public key is : {}", &ctx.accounts.new_account.key());

        let lamports = (Rent::get()?).minimum_balance(0);

        let create_sytems_account = CpiContext::new(
             ctx.accounts.system_program.to_account_info(),
             CreateAccount {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.new_account.to_account_info()
             }
        );
        
        create_account(create_sytems_account, lamports, 0, &ctx.accounts.system_program.key())?;

        msg!("account create successfully");

        Ok(())
    }


}

#[derive(Accounts)]
pub struct Hello {}

#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub signer : Signer<'info>,
    
    #[account(mut)]
    pub new_account : Signer<'info>,

    pub system_program : Program<'info, System>
}