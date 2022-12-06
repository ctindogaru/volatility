use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetStandardDeviationInPastDay<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub history_account_info: AccountInfo<'info>,
}
