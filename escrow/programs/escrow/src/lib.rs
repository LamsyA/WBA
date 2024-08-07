use anchor_lang::prelude::*;

declare_id!("BPSkKKbLFseoabHF5cwdih4V2PUbmnc44j488XSLGDm5");

mod contexts;
use contexts::*;
mod state;
use state::*;
#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)
    }
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()
    }
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
