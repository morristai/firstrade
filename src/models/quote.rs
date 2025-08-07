use chrono::serde::ts_milliseconds::deserialize as from_milli_ts;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTime {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<MarketTimeResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTimeResult {
    pub is_trading_date: bool,
    pub seconds_till_open: Option<i64>,
    pub seconds_since_close: Option<i64>,
    pub current_date: String,
    pub current_date_dash: String,
}

// ==================== Single Quote ====================
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

// ==================== Stock OHLC ====================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mohlc {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<HashMap<String, StockOhlc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockOhlc {
    pub ohlc: Vec<OhlcEntry>,
    pub vol: Vec<VolEntry>,
    pub prev_close: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhlcEntry(
    #[serde(deserialize_with = "from_milli_ts")] pub DateTime<Utc>,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolEntry(
    #[serde(deserialize_with = "from_milli_ts")] pub DateTime<Utc>,
    pub f64,
);

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

    #[test]
    fn test_stock_ohlc_response() {
        let raw_json = r#"{
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "result": {
                "XYZ": {
                    "ohlc": [
                        [
                            1754472600000,
                            40.13,
                            40.13,
                            39.99,
                            39.99
                        ],
                        [
                            1754472660000,
                            40.27,
                            40.27,
                            40.27,
                            40.27
                        ],
                        [
                            1754481060000,
                            41.19,
                            41.2,
                            41.19,
                            41.2
                        ],
                        [
                            1754492220000,
                            41.59,
                            41.59,
                            41.59,
                            41.59
                        ]
                    ],
                    "vol": [
                        [
                            1754472600000,
                            331
                        ],
                        [
                            1754472660000,
                            259
                        ],
                        [
                            1754481060000,
                            400
                        ],
                        [
                            1754492220000,
                            250
                        ]
                    ],
                    "prev_close": 40.34
                },
                "ABCD": {
                    "ohlc": [
                        [
                            1754472600000,
                            55.61,
                            55.69,
                            54.67,
                            54.73
                        ],
                        [
                            1754472660000,
                            54.75,
                            55.35,
                            54.12,
                            54.17
                        ],
                        [
                            1754475600000,
                            53.32,
                            53.39,
                            52.97,
                            52.97
                        ],
                        [
                            1754478600000,
                            52,
                            52.01,
                            51.91,
                            51.91
                        ],
                        [
                            1754481600000,
                            52.05,
                            52.06,
                            52,
                            52
                        ],
                        [
                            1754484600000,
                            51.18,
                            51.19,
                            51.11,
                            51.11
                        ],
                        [
                            1754487600000,
                            51.53,
                            51.53,
                            51.44,
                            51.44
                        ],
                        [
                            1754490600000,
                            50.93,
                            51.18,
                            50.93,
                            51.16
                        ],
                        [
                            1754493600000,
                            50.58,
                            50.61,
                            50.56,
                            50.6
                        ]
                    ],
                    "vol": [
                        [
                            1754472600000,
                            58347
                        ],
                        [
                            1754472660000,
                            21497
                        ],
                        [
                            1754475600000,
                            16685
                        ],
                        [
                            1754478600000,
                            4232
                        ],
                        [
                            1754481600000,
                            19443
                        ],
                        [
                            1754484600000,
                            2720
                        ],
                        [
                            1754487600000,
                            3544
                        ],
                        [
                            1754490600000,
                            3414
                        ],
                        [
                            1754493600000,
                            4192
                        ]
                    ],
                    "prev_close": 55.52
                }
            }
        }"#;

        let response: Mohlc = serde_json::from_str(raw_json).unwrap();
        assert_eq!(response.status_code, StatusCode::OK);

        if let Some(result) = response.result {
            assert!(result.contains_key("XYZ"));
            assert!(result.contains_key("ABCD"));
            let xyz_ohlc = &result["XYZ"];
            assert_eq!(xyz_ohlc.ohlc.len(), 4);
            assert_eq!(xyz_ohlc.vol.len(), 4);
            assert_eq!(xyz_ohlc.prev_close, 40.34);
        } else {
            panic!("Expected result to be present");
        }
    }
}
