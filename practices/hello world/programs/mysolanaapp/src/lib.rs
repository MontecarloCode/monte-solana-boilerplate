/* programs/mysolanaapp/src/lib.rs */
use anchor_lang::prelude::*;

declare_id!("6D9TGvtPxVJ3NvGGxfCpiT7oV5dK4iBRUg7ogDD9MPjk");

#[program]
mod mysolanaapp {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, data: String) -> Result<()> {
        let base_account = &mut _ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }

    pub fn update(_ctx: Context<Update>, data: String) -> Result<()> {
        let base_account = &mut _ctx.accounts.base_account;
        let copy = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub data: String,
    pub data_list: Vec<String>,
}
