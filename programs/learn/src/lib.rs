use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;

declare_id!("CAVZT8xhXfRKhZSBEJ884dvjpDsTaDZjjT3MxXNumbwj");

#[program]
pub mod learn {
      use super::*;  

      pub fn create_user(ctx : Context<CreateUser>, name: String) -> Result<()> {
          instructions::create_user(ctx, name)
      }
      pub fn close_user(ctx : Context<ClosUser>) -> Result<()> {
            instructions::close_user(ctx)
        }
}


