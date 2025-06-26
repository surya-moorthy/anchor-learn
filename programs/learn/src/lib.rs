use anchor_lang::prelude::*;

declare_id!("CAVZT8xhXfRKhZSBEJ884dvjpDsTaDZjjT3MxXNumbwj");

#[program]
pub mod learn {
    use crate::instruction::HelloWorld;

    use super::*;
  
    pub fn hello_world(_ctx : Context<Hello> )->Result<()> {
        msg!("Hello, World!");

        msg!("this is a basic program which has a programid : {}",&id());
        Ok(())
    } 
}

#[derive(Accounts)]
pub struct Hello {}