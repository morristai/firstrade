use chrono::serde::ts_milliseconds::deserialize as from_milli_ts;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== Market Time ====================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTimeResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<MarketTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTime {
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
    pub result: Option<QuoteResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QuoteResult {
    Stock(StockQuote),
    Option(OptionQuote),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionQuote {
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
pub struct StockQuote {
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
pub struct MohlcResponse {
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

// ==================== Stock Fundamental ====================
#[derive(Serialize, Deserialize)]
pub struct FundamentalResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<Fundamental>,
}

#[derive(Serialize, Deserialize)]
pub struct Fundamental {
    pub high: f64,
    pub low: f64,
    pub analyst_report: Option<AnalystReport>,
    pub industry: String,
    pub rating: Option<i64>,
    pub fairvalue: Option<i64>,
    pub consider_buy: Option<i64>,
    pub consider_sell: Option<i64>,
    pub uncertainty: Option<String>,
    pub economic_moat: Option<String>,
    pub stewardship: Option<String>,
    pub total_return_3m: f64,
    pub pe: Option<f64>,
    pub eps: f64,
    pub symbol: String,
    pub prev_close: f64,
    pub ms_exch: String,
    pub equity_type: String,
    pub hi_52wk: f64,
    pub hi_date_52wk: String,
    pub low_52wk: f64,
    pub low_date_52wk: String,
    pub dividend_yield: f64,
    pub beta: f64,
    pub mkt_cap: i64,
    pub shares_outstanding: i64,
    pub ex_date: String,
    pub avg_vol_1m: Option<i64>,
    pub mkt_return_1m: Option<i64>,
    pub yield_1y: Option<i64>,
    pub etn: Option<i64>,
    pub pgx_ratio: Option<i64>,
    pub pe_ratio: Option<f64>,
    pub pb_ratio: Option<f64>,
    pub open: f64,
    pub return_1y: Option<i64>,
    pub avg_vol_3m: i64,
    pub net_margin: f64,
    pub diluted_eps: f64,
    pub forward_pe: f64,
    pub dividend_ytd: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct AnalystReport {
    pub symbol: String,
    pub title: String,
    pub author: String,
    pub pdf_path: String,
    pub security_name: String,
    pub ts: i64,
}

// ==================== Company Profile ====================
#[derive(Serialize, Deserialize)]
pub struct CompanyProfileResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: CompanyProfile,
}

#[derive(Serialize, Deserialize)]
pub struct CompanyProfile {
    pub symbol: String,
    #[serde(rename = "ExchangeId")]
    pub exchange_id: String,
    pub profile: String,
}

// ==================== Cash Dividend ====================
#[derive(Serialize, Deserialize)]
pub struct CashDividendResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub items: CashDividend,
}

#[derive(Serialize, Deserialize)]
pub struct CashDividend {
    pub ex_date: String,
    pub pay_date: String,
    pub annual_amt: f64,
    #[serde(rename = "yield")]
    pub r#yield: f64,
    pub currency: String,
    pub amt: f64,
    pub frequency: String,
}

// ==================== Corp Calendar ====================
#[derive(Serialize, Deserialize)]
pub struct CorpCalendarResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    pub error: String,
    pub message: String,
    pub items: Vec<CorpCalendar>,
}

#[derive(Serialize, Deserialize)]
pub struct CorpCalendar {
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "EventType")]
    pub event_type: String,
    #[serde(rename = "EventURL")]
    pub event_url: String,
    #[serde(rename = "BeginDateTime")]
    pub begin_date_time: String,
    #[serde(rename = "EndDateTime")]
    pub end_date_time: String,
    #[serde(rename = "EstimatedDateforNextEvent")]
    pub estimated_datefor_next_event: Option<String>,
    #[serde(rename = "EventStatus")]
    pub event_status: String,
    #[serde(rename = "EventFiscalYear")]
    pub event_fiscal_year: String,
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,
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

        let response: MarketTimeResponse = serde_json::from_value(json_data).unwrap();
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
            QuoteResult::Stock(stock) => {
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
            QuoteResult::Option(option) => {
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

        let response: MohlcResponse = serde_json::from_str(raw_json).unwrap();
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

    #[test]
    fn test_fundamental_response() {
        let raw_json = r#"
        {
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "result": {
                "high": 220.85,
                "low": 216.58,
                "analyst_report": {
                    "symbol": "AAPL",
                    "title": "Apple: Another $100 Billion Looks to Be Enough to Avoid Tariffs",
                    "author": "William Kerwin",
                    "pdf_path": "https://invest.firstrade.com/ms/equity_reports/sr/2025/0P000000GY_20250807_RT.pdf",
                    "security_name": "Apple Inc",
                    "ts": 1754582449000
                },
                "industry": "Technology",
                "rating": 3,
                "fairvalue": 210,
                "consider_buy": null,
                "consider_sell": null,
                "uncertainty": "Medium",
                "economic_moat": "Wide",
                "stewardship": "Exemplary",
                "total_return_3m": 7.556294,
                "pe": 31.610000610351562,
                "eps": 6.59,
                "symbol": "AAPL",
                "prev_close": 213.25,
                "ms_exch": "126.1",
                "equity_type": "stock",
                "hi_52wk": 260.1,
                "hi_date_52wk": "20241226",
                "low_52wk": 169.21,
                "low_date_52wk": "20250408",
                "dividend_yield": 0.488,
                "beta": 1.16844,
                "mkt_cap": 3164713000000,
                "shares_outstanding": 14840390000000,
                "ex_date": "20250811",
                "avg_vol_1m": 37466639,
                "mkt_return_1m": null,
                "yield_1y": null,
                "etn": null,
                "pgx_ratio": null,
                "pe_ratio": 32.3596,
                "pb_ratio": 48.074,
                "open": 218.875,
                "return_1y": null,
                "avg_vol_3m": 54392453,
                "net_margin": 24.2961,
                "diluted_eps": 6.59,
                "forward_pe": 27.615331,
                "dividend_ytd": 0.77
            }
        }"#;
        let response: FundamentalResponse = serde_json::from_str(raw_json).unwrap();
        assert_eq!(response.status_code, StatusCode::OK);
        assert!(response.result.is_some());
        let result = response.result.unwrap();
        assert_eq!(result.symbol, "AAPL");
        assert_eq!(result.pe.unwrap(), 31.61000061035156);
        assert_eq!(result.eps, 6.59);
        assert_eq!(result.dividend_yield, 0.488);
    }
}
