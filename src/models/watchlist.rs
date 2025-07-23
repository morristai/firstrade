use http::StatusCode;
use serde::{Deserialize, Serialize};

// ==================== WatchList ====================
#[derive(Serialize, Deserialize)]
pub struct WatchLists {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub items: Option<Vec<WatchList>>,
}

#[derive(Serialize, Deserialize)]
pub struct WatchList {
    pub list_id: i64,
    pub name: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

// ==================== WatchListQuote ====================
#[derive(Serialize, Deserialize)]
pub struct WatchListQuote {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub result: Option<WatchListQuoteList>,
}

#[derive(Serialize, Deserialize)]
pub struct WatchListQuoteList {
    pub list_id: i64,
    pub name: String,
    pub list_items: Vec<ItemQuote>,
}

#[derive(Serialize, Deserialize)]
pub struct ItemQuote {
    pub watchlist_id: i64,
    pub sec_type: i64,
    pub symbol: String,
    pub quantity: i64,
    pub last: f64,
    pub bid: f64,
    pub ask: f64,
    pub vol: i64,
    pub change: f64,
    pub change_percent: f64,
    pub day_gain_amount: i64,
    pub unit_cost: i64,
    pub cost: i64,
    pub gain_amount: i64,
    pub gain_percent: i64,
    pub bidsize: i64,
    pub asksize: i64,
    pub high: f64,
    pub low: f64,
    pub close_price: f64,
    pub open_price: f64,
    pub update_time: String,
}

// ==================== AddWatchListResponse ====================

#[derive(Serialize, Deserialize)]
pub struct WatchListResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    #[serde(rename = "refCode")]
    pub ref_code: Option<i32>,
    pub result: Option<WatchListResult>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WatchListResult {
    AddNewWatchList(AddNewWatchList),
    AddNewSymbol(AddNewSymbol),
}

#[derive(Serialize, Deserialize)]
pub struct AddNewWatchList {
    pub list_id: i64,
    pub result: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddNewSymbol {
    pub watchlist_id: i64,
    pub result: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_delete_from_watchlist_deserialization() {
        let json_data = json!(
        {
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "result": {
                "watchlist_id": 55725063,
                "result": "success"
            }
        });
        let result: WatchListResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(result.status_code, StatusCode::OK);
        assert_eq!(result.error, "");
        assert_eq!(result.message, "Normal");
    }
}
