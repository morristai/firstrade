use super::account::PositionItem;
use crate::error::{Error, ErrorKind, Result};
use chrono::{FixedOffset, TimeZone, Utc};

pub fn parse_option_symbol(item: &mut PositionItem) -> Result<()> {
    // Validate minimum length (at least 14 chars for date+C/P+strike)
    if item.sec_type != 2 {
        return Err(Error::new(
            ErrorKind::Unexpected,
            "Invalid security type for option symbol",
        ));
    }

    // Find position where 6-digit date starts (YYMMDD)
    let mut found = false;
    let mut date_start = 0;
    let symbol = item.symbol.clone();
    for i in (0..=symbol.len() - 14).rev() {
        let date_part = &symbol[i..i + 6];
        let type_char = symbol.chars().nth(i + 6).unwrap_or(' ');
        let strike_part = symbol.get(i + 7..i + 15).unwrap_or("");

        if date_part.chars().all(|c| c.is_ascii_digit())
            && (type_char == 'C' || type_char == 'P')
            && strike_part.chars().all(|c| c.is_ascii_digit())
        {
            date_start = i;
            found = true;
            break;
        }
    }

    if !found {
        return Err(Error::new(
            ErrorKind::Unexpected,
            format!("Invalid option symbol format: {symbol}"),
        ));
    }

    item.symbol = symbol[..date_start].to_string();

    if symbol.chars().nth(date_start + 6).unwrap() == 'C' {
        item.is_call = Some(true);
    } else {
        item.is_call = Some(false);
    };
    let date_str = &symbol[date_start..date_start + 6];
    let strike_str = &symbol[date_start + 7..date_start + 15];

    // ======== Parse expiration date (YYMMDD) ========
    let (year, month, day) = (
        format!("20{}", &date_str[0..2]).parse::<i32>().map_err(|e| {
            Error::new(
                ErrorKind::Unexpected,
                format!("Failed to parse year in {symbol}: {e}"),
            )
        })?,
        date_str[2..4].parse::<u32>().map_err(|e| {
            Error::new(
                ErrorKind::Unexpected,
                format!("Failed to parse month in {symbol}: {e}"),
            )
        })?,
        date_str[4..6].parse::<u32>().map_err(|e| {
            Error::new(
                ErrorKind::Unexpected,
                format!("Failed to parse day in {symbol}: {e}"),
            )
        })?,
    );

    let est_offset = FixedOffset::west_opt(5 * 3600).unwrap();
    let expiration_date = est_offset.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();
    item.expiration_date = Some(expiration_date.with_timezone(&Utc));

    // ======= Parse strike price (8 digits, last 3 are decimals, e.g., 00100000 = 100.000) =======
    let strike_price = strike_str.parse::<u64>().map_err(|e| {
        Error::new(
            ErrorKind::Unexpected,
            format!("Failed to parse strike price in {symbol}: {e}"),
        )
    })? as f64
        / 1000.0;
    item.strike_price = Some(strike_price);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_option_symbol() {
        let json_data = json!({
            "quantity": -5,
            "last": 0.09,
            "bid": 0.09,
            "ask": 0.1,
            "vol": 1212,
            "close": 0.84,
            "cost": -2174.87,
            "unit_cost": 4.34974,
            "today_share": 0,
            "today_exe_price": 0,
            "sec_type": 2,
            "market_value": -45,
            "change": -0.75,
            "time": "14:25:47",
            "company_name": "UnitedHealth Group Incorporated (DE)",
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
            "asksize": 5,
            "bidsize": 5,
            "open_px": 0,
            "day_high": 0.4,
            "day_low": 0.07,
            "purchase_date": "",
            "day_held": 0,
            "adj_cost": -2174.87,
            "adj_unit_cost": 4.3497,
            "adj_gainloss": 2129.87,
            "adj_gainloss_percent": 97.9309,
            "change_percent": -89.2857,
            "drip": false,
            "loan": false,
            "gainloss": 2129.87,
            "gainloss_percent": 97.9309,
            "symbol": "UNH250822P00250000"
        });
        let mut item: PositionItem = serde_json::from_value(json_data).unwrap();
        parse_option_symbol(&mut item).unwrap();
        assert_eq!(item.symbol, "UNH");
        assert_eq!(
            item.expiration_date.unwrap().to_string(),
            "2025-08-22 05:00:00 UTC"
        );
        assert_eq!(item.strike_price.unwrap(), 250.0);
        assert_eq!(item.is_call.unwrap(), false);
    }
}
