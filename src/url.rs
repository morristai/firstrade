pub(crate) const FT_HOST: &str = "https://api3x.firstrade.com";
pub(crate) static ACCESS_TOKEN: &str = "833w3XuIFycv18ybi";
pub(crate) static USER_AGENT: &str = "okhttp/4.9.2";

#[inline(always)]
pub fn login() -> String {
    format!("{FT_HOST}/sess/login")
}

#[inline(always)]
pub fn verify_pin() -> String {
    format!("{FT_HOST}/sess/verify_pin")
}

// #[inline(always)]
// pub fn profile() -> String {
//     format!("{FT_HOST}/account/profile")
// }

// #[inline(always)]
// pub fn request_code() -> String {
//     format!("{FT_HOST}/sess/request_code")
// }

// #[inline(always)]
// pub fn jwt_auth() -> String {
//     format!("{FT_HOST}/private/tc/authenticate")
// }

#[inline(always)]
pub fn market_time() -> String {
    format!("{FT_HOST}/public/market_time")
}

#[inline(always)]
pub fn account_list() -> String {
    format!("{FT_HOST}/private/acct_list")
}

#[inline(always)]
pub fn user_info(account: &str) -> String {
    format!("{FT_HOST}/private/userinfo?account={account}")
}

#[inline(always)]
pub fn account_balances(account: &str) -> String {
    format!("{FT_HOST}/private/balances?account={account}")
}

#[inline(always)]
pub fn account_history(account: &str, range: &str, page: u32, per_page: u32) -> String {
    format!(
        "{FT_HOST}/private/account_history?range={range}&page={page}&account={account}&per_page={per_page}"
    )
}

#[inline(always)]
pub fn account_positions(account: &str) -> String {
    format!("{FT_HOST}/private/positions?account={account}&per_page=200")
}

#[inline(always)]
pub fn watchlists() -> String {
    format!("{FT_HOST}/private/watchlists")
}

#[inline(always)]
pub fn watchlist() -> String {
    format!("{FT_HOST}/private/watchlist")
}

#[inline(always)]
pub fn watchlist_quote(id: u32) -> String {
    format!("{FT_HOST}/private/watchlists/{id}")
}

#[inline(always)]
pub fn single_quote(account: &str, symbol: &str) -> String {
    format!("{FT_HOST}/public/quote?account={account}&q={symbol}")
}

// NOTE: symbols should be a comma-separated list of symbols
// e.g. "AAPL,GOOGL,MSFT"
// resolution must be less than or equal to 50
#[inline(always)]
pub fn stock_mohlc(symbols: &str, resolution: u8) -> String {
    format!("{FT_HOST}/public/mohlc?symbols={symbols}&resolution={resolution}")
}
