use switchboard_v2::SwitchboardDecimal;

const DEFAULT_DECIMALS: u32 = 6;

pub fn to_default_decimals(price: &SwitchboardDecimal) -> u64 {
    if price.scale < DEFAULT_DECIMALS {
        let diff = DEFAULT_DECIMALS - price.scale;
        return price.mantissa as u64 * 10u64.pow(diff);
    } else {
        let diff = price.scale - DEFAULT_DECIMALS;
        return price.mantissa as u64 / 10u64.pow(diff);
    }
}
