pub mod context;
pub mod utils;
use anchor_lang::prelude::*;
use context::*;
use switchboard_v2::AggregatorHistoryBuffer;
use utils::*;

const SECONDS_IN_HOUR: i64 = 3_600;

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

        let mut prices = [0u64; 24];
        for day in 0..24 {
            let previous_timestamp = current_timestamp - SECONDS_IN_HOUR * day;
            let price = history_buffer
                .lower_bound(previous_timestamp)
                .unwrap()
                .value;
            prices[day as usize] = to_default_decimals(&price);
        }

        msg!("Result {:?}!", prices);

        Ok(())
    }
}
