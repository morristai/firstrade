use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ==================== Account List ====================
#[derive(Serialize, Deserialize)]
pub struct AccountList {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub items: Option<Vec<AccountItems>>,
    pub grand_total: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct AccountItems {
    pub account: String,
    pub alias: String,
    pub permissions: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub ext_hours_trading_status: String,
    pub signed_fractional: String,
    pub total_value: f64,
    pub option_level: i64,
    pub default: bool,
}

// =================== User Info ====================
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub sid: String,
    pub ftat: String,
    pub onbehalf_id: String,
    pub eui: String,
    pub realtime_quote: bool,
    pub streaming_quote: bool,
    pub real_time_index_quotes_status: bool,
    pub real_time_watchlist_quote: bool,
    pub nls_quote: bool,
    pub authenticated: bool,
    pub accounts: Vec<String>,
    pub primary_accounts: Vec<String>,
    pub admin_accounts: Vec<String>,
    pub selected_account: String,
    pub locale: String,
    pub menu: Menu,
    pub edoc: Edoc,
}

#[derive(Serialize, Deserialize)]
pub struct Edoc {
    pub show_reminder: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Menu {
    pub promotion: bool,
    pub contact: bool,
    pub funding: bool,
    pub terms: bool,
    pub tutorials: bool,
    pub acats: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub page: u32,
    pub pages: u32,
    pub per_page: u32,
    pub total: u32,
    pub realtime: String,
    pub items: Vec<PositionItem>,
    pub total_market_value: f64,
    pub total_gainloss: f64,
    pub total_gainloss_percent: f64,
    pub total_daychange_amount: f64,
    pub total_daychange_percent: f64,
    #[serde(rename = "isCostBasisReady")]
    pub is_cost_basis_ready: bool,
    pub account: String,
    pub pagination: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionItem {
    pub quantity: i32,
    pub last: f64,
    pub bid: f64,
    pub ask: f64,
    pub vol: u32,
    pub close: f64,
    pub cost: f64,
    pub unit_cost: f64,
    pub today_share: u32,
    pub today_exe_price: f64,
    pub sec_type: u8,
    pub market_value: f64,
    pub change: f64,
    pub time: String,
    pub company_name: String,
    pub avg_vol: u32,
    pub eps: f64,
    pub pe: f64,
    pub div_share: f64,
    pub r#yield: f64,
    pub ex_div_date: String,
    pub div_date: String,
    pub market_cap: u64,
    #[serde(rename = "5yr_growth")]
    pub five_yr_growth: f64,
    pub beta: f64,
    pub annual_div_rate: f64,
    #[serde(rename = "52w_high")]
    pub fifty_two_w_high: f64,
    #[serde(rename = "52w_low")]
    pub fifty_two_w_low: f64,
    pub has_lots: bool,
    pub asksize: u32,
    pub bidsize: u32,
    pub open_px: f64,
    pub day_high: f64,
    pub day_low: f64,
    pub purchase_date: String,
    pub day_held: u32,
    pub adj_cost: f64,
    pub adj_unit_cost: f64,
    pub adj_gainloss: f64,
    pub adj_gainloss_percent: f64,
    pub change_percent: f64,
    pub drip: bool,
    pub loan: bool,
    pub gainloss: f64,
    pub gainloss_percent: f64,
    pub symbol: String,
    // NOTE: for option parse result
    pub expiration_date: Option<DateTime<Utc>>,
    pub strike_price: Option<f64>,
    pub is_call: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<BalanceResult>,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceResult {
    pub account: String,
    pub freetrade_count: i64,
    pub last_freetrade_date: String,
    pub total_account_value: f64,
    pub total_account_change: f64,
    pub long_stock_change: f64,
    pub short_stock_change: f64,
    pub long_option_change: f64,
    pub short_option_change: f64,
    pub fixed_income_change: f64,
    pub mutual_funds_change: f64,
    pub cash_balance: f64,
    pub cash_balance_change: f64,
    pub margin_balance: f64,
    pub margin_balance_change: f64,
    pub margin_buying_power: f64,
    pub long_stock_value: f64,
    pub long_option_value: f64,
    pub short_option_value: f64,
    pub non_margin_buying_power: f64,
    pub daytrade_buying_power: f64,
    pub money_locked_by_pending_orders: f64,
}

// =================== Account History ====================
#[derive(Serialize, Deserialize)]
pub struct AccountHistory {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub items: Option<Vec<HistoryItem>>,
    pub per_page: u32,
    pub page: u32,
    pub total: u32,
}

#[derive(Serialize, Deserialize)]
pub struct HistoryItem {
    pub report_date: String,
    pub trans_str: String,
    pub quantity: i64,
    pub trade_price: f64,
    pub amount: f64,
    pub description: String,
    #[serde(rename = "descriptionArray")]
    pub description_array: Vec<String>,
    pub symbol: String,
    pub account_type: String,
    #[serde(rename = "shortDesc")]
    pub short_desc: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_account_deserialization() {
        let json_data = json!(
                    {
            "sid": "5DB135DCE6194D2F0373B862759870AA902662BC6D32231443654DD8A179E123",
            "ftat": "516888290AD23C74C4F989481C312B1B545FFE4CCE37A656E73F5DDB99B39900",
            "onbehalf_id": "FT00012345",
            "eui": "A_9A32677D15A930269279AA9F09FADB263F34859556C0D9901062C9B18318D123",
            "realtime_quote": true,
            "streaming_quote": false,
            "real_time_index_quotes_status": true,
            "real_time_watchlist_quote": true,
            "nls_quote": true,
            "authenticated": true,
            "accounts": [
                "12345678"
            ],
            "primary_accounts": [
                "12345678"
            ],
            "admin_accounts": [
                "12345678"
            ],
            "selected_account": "12345678",
            "locale": "en-us",
            "menu": {
                "promotion": false,
                "contact": true,
                "funding": true,
                "terms": true,
                "tutorials": true,
                "acats": true
            },
            "edoc": {
                "show_reminder": true
            }
        }
                );

        let account_info: UserInfo = serde_json::from_value(json_data).unwrap();
        assert_eq!(
            account_info.sid,
            "5DB135DCE6194D2F0373B862759870AA902662BC6D32231443654DD8A179E123"
        );
        assert!(account_info.realtime_quote);
        assert_eq!(account_info.accounts.len(), 1);
        assert_eq!(account_info.accounts[0], "12345678");
    }

    #[test]
    fn test_position_deserialization() {
        let json_data = json!(
        {
          "statusCode": 200,
          "error": "",
          "message": "Normal",
          "page": 1,
          "pages": 1,
          "per_page": 200,
          "total": 31,
          "realtime": "T",
          "items": [
            {
              "quantity": 10,
              "last": 0.9,
              "bid": 0.85,
              "ask": 1.6,
              "vol": 830,
              "close": 0.9,
              "cost": 1050.29,
              "unit_cost": 1.05029,
              "today_share": 0,
              "today_exe_price": 0,
              "sec_type": 2,
              "market_value": 900,
              "change": 0,
              "time": "16:00:00",
              "company_name": "ABCD Inc.",
              "avg_vol": 0,
              "eps": 0,
              "pe": 0,
              "div_share": 0,
              "yield": 0,
              "ex_div_date": "",
              "div_date": "",
              "market_cap": 0,
              "5yr_growth": 0,
              "beta": 0,
              "annual_div_rate": 0,
              "52w_high": 0,
              "52w_low": 0,
              "has_lots": false,
              "asksize": 20,
              "bidsize": 1302,
              "open_px": 0,
              "day_high": 1.05,
              "day_low": 0.9,
              "purchase_date": "",
              "day_held": 0,
              "adj_cost": 1050.29,
              "adj_unit_cost": 1.0503,
              "adj_gainloss": -150.29,
              "adj_gainloss_percent": -14.3094,
              "change_percent": 0,
              "drip": false,
              "loan": false,
              "gainloss": -150.29,
              "gainloss_percent": -14.3094,
              "symbol": "ABCD260116C00003000"
            },
            {
              "quantity": 15,
              "last": 39.63,
              "bid": 39.46,
              "ask": 39.59,
              "vol": 12867180,
              "close": 40.1,
              "cost": 597,
              "unit_cost": 39.8,
              "today_share": 0,
              "today_exe_price": 0,
              "sec_type": 1,
              "market_value": 594.45,
              "change": -0.47,
              "time": "00:55:40",
              "company_name": "Apple Inc.",
              "avg_vol": 14136023,
              "eps": 0,
              "pe": 0,
              "div_share": 0,
              "yield": 0,
              "ex_div_date": "",
              "div_date": "",
              "market_cap": 13754,
              "5yr_growth": 0,
              "beta": 2.0115,
              "annual_div_rate": 0,
              "52w_high": 42.93,
              "52w_low": 9.32,
              "has_lots": false,
              "asksize": 117,
              "bidsize": 231,
              "open_px": 0,
              "day_high": 41.6,
              "day_low": 38.82,
              "purchase_date": "",
              "day_held": 0,
              "adj_cost": 597,
              "adj_unit_cost": 39.8,
              "adj_gainloss": -2.55,
              "adj_gainloss_percent": -0.4271,
              "change_percent": -1.1721,
              "drip": false,
              "loan": false,
              "gainloss": -2.55,
              "gainloss_percent": -0.4271,
              "symbol": "AAPL"
            }
          ],
          "total_market_value": 424250.36,
          "total_gainloss": 129421.736,
          "total_gainloss_percent": 43.8973,
          "total_daychange_amount": 6961.45,
          "total_daychange_percent": 1.6683,
          "isCostBasisReady": true,
          "account": "12345678",
          "pagination": {}
        });

        let position: Position = serde_json::from_value(json_data).unwrap();
        assert_eq!(position.status_code, 200);
        assert_eq!(position.items.len(), 2);
        assert_eq!(position.items[0].symbol, "ABCD260116C00003000");
        assert_eq!(position.items[1].symbol, "AAPL");
        assert_eq!(position.total_market_value, 424250.36);
        assert_eq!(position.total_gainloss_percent, 43.8973);
    }

    #[test]
    fn test_balance_deserialization() {
        let json_data = json!(
                            {
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "result": {
                "account": "63882268",
                "freetrade_count": 0,
                "last_freetrade_date": "",
                "total_account_value": 12345678.11,
                "total_account_change": 8652.99,
                "long_stock_change": 8652.99,
                "short_stock_change": 0,
                "long_option_change": 0,
                "short_option_change": 0,
                "fixed_income_change": 0,
                "mutual_funds_change": 0,
                "cash_balance": 12345.67,
                "cash_balance_change": 0,
                "margin_balance": -123.11,
                "margin_balance_change": 123,
                "margin_buying_power": 385708.77,
                "long_stock_value": 567890.11,
                "long_option_value": 56789.11,
                "short_option_value": -12345,
                "non_margin_buying_power": 192854.39,
                "daytrade_buying_power": 500000.11,
                "money_locked_by_pending_orders": 0
            }
        }
                        );
        let balance: Balance = serde_json::from_value(json_data).unwrap();
        assert_eq!(balance.status_code, 200);
        let result = balance.result.unwrap();
        assert_eq!(result.cash_balance, 12345.67);
        assert_eq!(result.long_stock_change, 8652.99);
        assert_eq!(result.daytrade_buying_power, 500000.11);
        assert_eq!(result.margin_balance, -123.11);
    }
}
