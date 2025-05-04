use anchor_lang::prelude::*;
use utils::*;

mod utils;

declare_id!("Hz1rgv8CaJorxqGsMpuYLCcZJ9BNZEeghC7mNB4Wodsm");

#[program]
pub mod solana_pda {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        message: String,
        pda_id: u32,
    ) -> Result<()> {
        msg!("Create Message: {}", message);
        let account_data = &mut ctx.accounts.message_account;
        account_data.id = pda_id;
        account_data.user = ctx.accounts.user.key();
        account_data.message = message;
        account_data.bump = ctx.bumps.message_account;
        Ok(())
    }

    pub fn update(
        ctx: Context<Update>,
        new_message: String,
        _pda_id: u32,
    ) -> Result<()> {
        msg!("Update Message: {}", new_message);
        let account_data = &mut ctx.accounts.message_account;
        account_data.message = new_message;
        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(message: String, pda_id: u32)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [
            b"message",
            pda_id.to_string().as_bytes(),
            user.key().as_ref(),
        ],
        bump,
        payer = user,
        space = calculate_account_storage(AccountStorageParams{
            id: &pda_id,
            message: &message,
        }),
    )]
    pub message_account: Account<'info, MessageAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(message: String, pda_id: u32)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"message",
            &pda_id.to_string().as_bytes(),
            user.key().as_ref(),
        ],
        bump = message_account.bump,
        realloc = calculate_account_storage(AccountStorageParams{
            id: &pda_id,
            message: &message,
        }),
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, MessageAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete {}

#[account]
pub struct MessageAccount {
    pub id: u32,
    pub user: Pubkey,
    pub message: String,
    pub bump: u8,
}
