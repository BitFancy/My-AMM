use anchor_lang::prelude::*;

declare_id!("5DiJukQabBocT5YZ82ftKd6GKioC8FTEwqxpBshsTV7C");

#[program]
pub mod my_ammsol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_x: u64, amount_y: u64) -> ProgramResult {
        let liquidity_pool = &mut ctx.accounts.liquidity_pool;
        liquidity_pool.asset_x += amount_x;
        liquidity_pool.asset_y += amount_y;
        liquidity_pool.constant = liquidity_pool.asset_x * liquidity_pool.asset_y;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct LiquidityPool {
    pub asset_x: u64,
    pub asset_y: u64,
    pub constant: u64,
}
