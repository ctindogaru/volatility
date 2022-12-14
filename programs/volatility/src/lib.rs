pub mod context;
pub mod utils;
use anchor_lang::prelude::*;
use context::*;
use spl_math::approximations::sqrt;
use switchboard_v2::AggregatorHistoryBuffer;
use utils::*;

const SECONDS_IN_HOUR: i64 = 3_600;

declare_id!("FUWQwWXaBiu1h6b8tmTv2H1d4pvXD342ytC5vR3dwEX4");

#[program]
pub mod volatility {
    use super::*;

    pub fn get_standard_deviation_for_past_day(
        ctx: Context<GetStandardDeviationForPastDay>,
    ) -> Result<()> {
        let history_buffer = AggregatorHistoryBuffer::new(&ctx.accounts.history_account_info)?;
        let current_timestamp = Clock::get()?.unix_timestamp;

        let mut prices = [0u64; 24];
        let mut average_price = 0u64;

        // Step 1: Take 24 samples from the last 24 hours.
        for hour in 0..24 {
            let previous_timestamp = current_timestamp - SECONDS_IN_HOUR * hour;
            let price = history_buffer
                .lower_bound(previous_timestamp)
                .unwrap()
                .value;
            prices[hour as usize] = to_default_decimals(&price);
            average_price += prices[hour as usize];
        }

        // Step 2: Find the average price in the last 24h.
        average_price /= 24;

        let mut average_deviation = 0u64;
        // Step 3: Compute the deviation for each sample.
        for hour in 0..24 {
            let deviation = prices[hour] as i64 - average_price as i64;
            prices[hour] = deviation.pow(2) as u64;
            average_deviation += prices[hour];
        }

        // Step 4: Compute the average deviation.
        average_deviation /= 24;

        // Step 5: Compute the standard deviation. Note that the result will have a precision of 2.
        let standard_deviation = sqrt(average_deviation).unwrap();
        msg!("Standard deviation: {:?}", standard_deviation);

        Ok(())
    }
}
