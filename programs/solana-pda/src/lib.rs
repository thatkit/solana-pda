use anchor_lang::prelude::*;

declare_id!("EFMWmbXQqyFeDWtXvARgUZYF8WUHrsTMNY7fTJ27rYzq");

#[program]
pub mod solana_pda {
    use super::*;

    pub fn create(ctx: Context<CreatePDA>) -> Result<()> {
        Ok(())
    }

    pub fn get(ctx: Context<GetPDA>) -> Result<()> {
        Ok(())
    }

    pub fn update(ctx: Context<UpdatePDA>) -> Result<()> {
        Ok(())
    }

    pub fn delete(ctx: Context<DeletePDA>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePDA {}

#[derive(Accounts)]
pub struct GetPDA {}

#[derive(Accounts)]
pub struct UpdatePDA {}

#[derive(Accounts)]
pub struct DeletePDA {}
