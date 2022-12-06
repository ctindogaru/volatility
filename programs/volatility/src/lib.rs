use anchor_lang::prelude::*;

declare_id!("FUWQwWXaBiu1h6b8tmTv2H1d4pvXD342ytC5vR3dwEX4");

#[program]
pub mod volatility {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
