use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MarketTime {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<MarketTimeResult>,
}

#[derive(Serialize, Deserialize)]
pub struct MarketTimeResult {
    pub is_trading_date: bool,
    pub seconds_till_open: Option<i64>,
    pub seconds_since_close: Option<i64>,
    pub current_date: String,
    pub current_date_dash: String,
}

// ==================== Quote Response ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleQuoteResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<Result>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Result {
    Stock(StockResult),
    Option(OptionResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionResult {
    pub symbol: String,
    pub sec_type: u8,
    pub underlying_symbol: String,
    pub class: String,
    pub tick: String,
    pub strike_price: f64,
    pub exp_date: String,
    pub bid: f64,
    pub bid_size: u32,
    pub ask: f64,
    pub ask_size: u32,
    pub last: f64,
    pub change: f64,
    pub high: f64,
    pub low: f64,
    pub change_color: String,
    pub vol: u32,
    pub quote_time: String,
    pub exchange: String,
    pub realtime: String,
    pub shares: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockResult {
    pub symbol: String,
    pub sec_type: u8,
    pub tick: String,
    pub bid: f64,
    pub bid_size: u32,
    pub ask: f64,
    pub ask_size: u32,
    pub last: f64,
    pub change: f64,
    pub high: f64,
    pub low: f64,
    pub bid_mmid: String,
    pub ask_mmid: String,
    pub last_mmid: String,
    pub last_size: u32,
    pub change_color: String,
    pub vol: u64,
    pub today_close: f64,
    pub prev_close: f64,
    pub show_close: String,
    pub change_percent: f64,
    pub margin_long_req: f64,
    pub margin_short_req: f64,
    pub open: f64,
    pub quote_time: String,
    pub last_trade_time: String,
    pub company_name: String,
    pub mssecid: String,
    pub exchange: String,
    pub has_option: bool,
    pub is_etf: bool,
    pub is_fractional: bool,
    pub is_overnight: bool,
    pub realtime: String,
    pub nls: String,
    pub shares: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_market_time_response() {
        let json_data = json!({
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "result": {
                "is_trading_date": true,
                "seconds_till_close": 18979,
                "current_date": "20250731",
                "current_date_dash": "2025-07-31"
            }
        });

        let response: MarketTime = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.status_code, StatusCode::OK);
        assert!(response.result.unwrap().is_trading_date);
    }

    #[test]
    fn test_stock_quote_response() {
        let json_data = json!({
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "result": {
                "symbol": "HIMS",
                "sec_type": 1,
                "tick": "N",
                "bid": 56.43,
                "bid_size": 900,
                "ask": 56.78,
                "ask_size": 207,
                "last": 56.739,
                "change": -0.121,
                "high": 57.21,
                "low": 54.6152,
                "bid_mmid": "BLUE",
                "ask_mmid": "BLUE",
                "last_mmid": "XADF",
                "last_size": 3000,
                "change_color": "red",
                "vol": 27510424,
                "today_close": 56.86,
                "prev_close": 56.33,
                "show_close": "Y",
                "change_percent": -0.21,
                "margin_long_req": 50,
                "margin_short_req": 50,
                "open": 0,
                "quote_time": "08:00:12 pm",
                "last_trade_time": "07:59 pm",
                "company_name": "Hims & Hers Health Inc. Class A",
                "mssecid": "",
                "exchange": "NYSE",
                "has_option": true,
                "is_etf": false,
                "is_fractional": true,
                "is_overnight": true,
                "realtime": "F",
                "nls": "F",
                "shares": 0
            }
        });

        let response: SingleQuoteResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.status_code, 200);
        match response.result.unwrap() {
            Result::Stock(stock) => {
                assert_eq!(stock.symbol, "HIMS");
                assert_eq!(stock.sec_type, 1);
                assert_eq!(stock.exchange, "NYSE");
            }
            _ => panic!("Expected StockResult"),
        }
    }

    #[test]
    fn test_option_quote_response() {
        let json_data = json!({
         "statusCode": 200,
         "error": "",
         "message": "Normal",
         "result": {
             "symbol": "OSCR270115C00010000",
             "sec_type": 2,
             "underlying_symbol": "OSCR",
             "class": "CALL",
             "tick": "N",
             "strike_price": 10,
             "exp_date": "20270115",
             "bid": 8.8,
             "bid_size": 17,
             "ask": 9.3,
             "ask_size": 238,
             "last": 9.2,
             "change": 1.3,
             "high": 9.2,
             "low": 8.72,
             "change_color": "green",
             "vol": 26,
             "quote_time": "03:38:57 pm",
             "exchange": "OPRA",
             "realtime": "F",
             "shares": 0
         }
        });
        let response: SingleQuoteResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.status_code, 200);

        match response.result.unwrap() {
            Result::Option(option) => {
                assert_eq!(option.symbol, "OSCR270115C00010000");
                assert_eq!(option.sec_type, 2);
                assert_eq!(option.underlying_symbol, "OSCR");
                assert_eq!(option.class, "CALL");
                assert_eq!(option.exchange, "OPRA");
            }
            _ => panic!("Expected OptionResult"),
        }
    }
}
