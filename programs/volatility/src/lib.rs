pub mod context;
use anchor_lang::prelude::*;
use context::*;
use std::convert::TryInto;
use switchboard_v2::AggregatorHistoryBuffer;

declare_id!("FUWQwWXaBiu1h6b8tmTv2H1d4pvXD342ytC5vR3dwEX4");

#[program]
pub mod volatility {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn calculate_volatility(ctx: Context<CalculateVolatility>) -> Result<()> {
        let history_buffer = AggregatorHistoryBuffer::new(&ctx.accounts.history_account_info)?;
        let current_timestamp = Clock::get()?.unix_timestamp;
        let one_hour_ago: f64 = history_buffer
            .lower_bound(current_timestamp - 3600)
            .unwrap()
            .value
            .try_into()?;
        msg!("Result {:?}!", one_hour_ago);

        Ok(())
    }
}
