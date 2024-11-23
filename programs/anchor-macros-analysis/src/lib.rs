use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("3t3f2Hp6Mw2sFqFxJcMuHj3RSWL6X4J9cBosSydfttKn");

#[program]
pub mod admin_user_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_timestamp: u64) -> Result<()> {
        let admin_account = &mut ctx.accounts.admin_account;
        admin_account.admin = *ctx.accounts.signer.key;
        admin_account.init_timestamp = init_timestamp;
        Ok(())
    }

    pub fn create_user(ctx: Context<CreateUser>, user_key: Pubkey, initial_amount: u64) -> Result<()> {
        let admin_account = &ctx.accounts.admin_account;
        if admin_account.admin != *ctx.accounts.signer.key {
            return Err(error!(ErrorCode::UnauthorizedAdmin));
        }

        let user_account = &mut ctx.accounts.user_account;
        user_account.user = user_key;
        user_account.amount = initial_amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 // Discriminator + Pubkey + u64
    )]
    pub admin_account: Account<'info, Admin>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub admin_account: Account<'info, Admin>,
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 8 // Discriminator + Pubkey + u64
    )]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Admin {
    pub admin: Pubkey,
    pub init_timestamp: u64,
}

#[account]
pub struct User {
    pub user: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Signer is not the authorized admin.")]
    UnauthorizedAdmin,
}
