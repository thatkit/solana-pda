use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
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

    /// pay-to-update -> storing lamports as in a vault
    pub fn update(
        ctx: Context<Update>,
        new_message: String,
        _pda_id: u32,
    ) -> Result<()> {
        msg!("Update Message: {}", new_message);

        let transfer_accounts = Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.vault_account.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        );

        transfer(cpi_context, 1_000_000)?;

        let account_data = &mut ctx.accounts.message_account;
        account_data.message = new_message;

        Ok(())
    }

    /// refund-on-delete
    pub fn delete(ctx: Context<Delete>, _pda_id: u32) -> Result<()> {
        msg!("Delete Message");

        let user_key = ctx.accounts.user.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"vault", user_key.as_ref(), &[ctx.bumps.vault_account]]];

        let transfer_accounts = Transfer {
            from: ctx.accounts.vault_account.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            transfer_accounts,
        ).with_signer(signer_seeds);

        transfer(cpi_context, ctx.accounts.vault_account.lamports())?;

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

    // a single vault account for each unique user
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(pda_id: u32)]
pub struct Delete<'info> {
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
        close = user,
    )]
    pub message_account: Account<'info, MessageAccount>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MessageAccount {
    pub id: u32,
    pub user: Pubkey,
    pub message: String,
    pub bump: u8,
}
