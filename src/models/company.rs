use http::StatusCode;
use serde::{Deserialize, Serialize};

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
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
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
    use http::StatusCode;

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
