use crate::error::{Error, Result};
use chrono::{DateTime, TimeZone};
use chrono_tz::{America::New_York, Tz};

#[derive(Debug, PartialEq)]
pub struct OptionData {
    pub ticker: String,
    pub expiration_date: DateTime<Tz>,
    pub strike_price: f64,
    pub option_type: String,
}

pub fn parse_option_symbol(symbol: &String) -> Result<OptionData, Error> {
    // Validate minimum length (at least 14 chars for date+C/P+strike)
    if symbol.len() < 14 {
        return Err(Error::Custom("Option symbol too short".to_string()));
    }

    // Find position where 6-digit date starts (YYMMDD)
    let mut found = false;
    let mut date_start = 0;
    for i in (0..=symbol.len() - 14).rev() {
        let date_part = &symbol[i..i + 6];
        let type_char = symbol.chars().nth(i + 6).unwrap_or(' ');
        let strike_part = &symbol.get(i + 7..i + 15).unwrap_or("");

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
        return Err(Error::Custom(format!(
            "Invalid option symbol format: {symbol}"
        )));
    }

    // Extract components
    let ticker = symbol[..date_start].to_string();
    let date_str = &symbol[date_start..date_start + 6];
    let option_type = if symbol.chars().nth(date_start + 6).unwrap() == 'C' {
        "Call".to_string()
    } else {
        "Put".to_string()
    };
    let strike_str = &symbol[date_start + 7..date_start + 15];

    // Parse expiration date (YYMMDD)
    let (year, month, day) = (
        format!("20{}", &date_str[0..2])
            .parse::<i32>()
            .map_err(|e| Error::Custom(format!("Failed to parse year in {symbol}: {e}")))?,
        date_str[2..4]
            .parse::<u32>()
            .map_err(|e| Error::Custom(format!("Failed to parse month in {symbol}: {e}")))?,
        date_str[4..6]
            .parse::<u32>()
            .map_err(|e| Error::Custom(format!("Failed to parse day in {symbol}: {e}")))?,
    );

    let expiration_date = New_York
        .with_ymd_and_hms(year, month, day, 0, 0, 0)
        .single()
        .ok_or_else(|| Error::Custom(format!("Invalid date in option symbol: {symbol}")))?;

    // Parse strike price (8 digits, last 3 are decimals, e.g., 00100000 = 100.000)
    let strike_price = strike_str
        .parse::<u64>()
        .map_err(|e| Error::Custom(format!("Failed to parse strike price in {symbol}: {e}")))?
        as f64
        / 1000.0;

    Ok(OptionData {
        ticker,
        expiration_date,
        strike_price,
        option_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_option_symbol() {
        let symbol = "AMD270115C00100000".to_string();
        let result = parse_option_symbol(&symbol).unwrap();
        assert_eq!(result.ticker, "AMD");
        assert_eq!(
            result.expiration_date.to_string(),
            "2027-01-15 00:00:00 EST"
        );
        assert_eq!(result.strike_price, 100.0);
        assert_eq!(result.option_type, "Call");
    }

    #[test]
    fn test_invalid_symbol() {
        let symbol = "INVALID".to_string();
        let result = parse_option_symbol(&symbol);
        assert!(result.is_err());
    }
}
