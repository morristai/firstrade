use http::StatusCode;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub(crate) struct LoginMfaRequest {
    #[serde(rename = "mfaCode")]
    pub mfa_code: String,
    pub remember_for: i64,
    pub t_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, TypedBuilder)]
pub(crate) struct LoginOtpRequest {
    #[serde(rename = "optCode")]
    pub otp_code: String,
    pub remember_for: i64,
    pub t_token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum LoginResponse {
    Otp(LoginNeedOtpResponse),
    Mfa(LoginNeedMfaResponse),
    Verify(LoginVerifiedResponse),
}

// ================= LoginWithOtpResponse =================
#[derive(Serialize, Deserialize)]
pub struct Fallback {
    pub strategy: String,
    #[serde(rename = "pinDisabled")]
    pub pin_disabled: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OtpOptions {
    #[serde(rename = "recipientId")]
    pub recipient_id: i64,
    pub channel: String,
    #[serde(rename = "recipientMask")]
    pub recipient_mask: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginNeedOtpResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub t_token: String,
    pub mfa: bool,
    pub otp: Vec<OtpOptions>,
    pub fallback: Fallback,
}

// ================= LoginWithMfaResponse =================
#[derive(Serialize, Deserialize)]
pub(crate) struct LoginNeedMfaResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub t_token: String,
    pub mfa: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LoginVerifiedResponse {
    #[serde(rename = "statusCode")]
    #[serde(with = "http_serde::status_code")]
    pub status_code: StatusCode,
    pub error: String,
    pub message: String,
    pub sid: String,
    pub ftat: String,
    pub onbehalf_id: String,
    pub eui: String,
    pub realtime_quote: bool,
    pub streaming_quote: bool,
    pub real_time_index_quotes_status: bool,
    pub real_time_watchlist_quote: bool,
    pub nls_quote: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TcResponse {
    #[serde(rename = "Authorization")]
    pub authorization: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_login_otp_deserialization() {
        let json_data = json!(
        {
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "t_token": "1f4b31f064cb12366e397192db86fcbf",
            "mfa": false,
            "otp": [
            {
                "recipientId": 354687,
                "channel": "sms",
                "recipientMask": "+1******0811"
            },
            {
                "recipientId": 120129,
                "channel": "email",
                "recipientMask": "l****@g****.com"
            }
            ],
            "fallback": {
            "strategy": "pin",
            "pinDisabled": true
        }
        }
        );
        let response: LoginResponse = serde_json::from_value(json_data).unwrap();
        if let LoginResponse::Otp(otp_response) = response {
            assert_eq!(otp_response.status_code, 200);
            assert_eq!(otp_response.error, "");
            assert_eq!(otp_response.message, "Normal");
            assert_eq!(otp_response.t_token, "1f4b31f064cb12366e397192db86fcbf");
            assert_eq!(otp_response.mfa, false);
            assert_eq!(otp_response.otp.len(), 2);
            assert_eq!(otp_response.fallback.strategy, "pin");
            assert!(otp_response.fallback.pin_disabled);
        } else {
            panic!("Expected LoginResponse::Otp");
        }
    }

    #[test]
    fn test_longin_mfa_deserialization() {
        let json_data = json!(
        {
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "t_token": "073e8a2ae7331c32e8b0c12004248e00",
            "mfa": true
        });

        let response: LoginResponse = serde_json::from_value(json_data).unwrap();
        if let LoginResponse::Mfa(init_response) = response {
            assert_eq!(init_response.status_code, 200);
            assert_eq!(init_response.error, "");
            assert_eq!(init_response.message, "Normal");
            assert_eq!(init_response.t_token, "073e8a2ae7331c32e8b0c12004248e00");
            assert_eq!(init_response.mfa, true);
        } else {
            panic!("Expected LoginResponse::Init");
        }
    }

    #[test]
    fn test_login_verified_deserialization() {
        let json_data = json!(
            {
            "statusCode": 200,
            "error": "",
            "message": "Normal",
            "sid": "87DEF0CF1D54F7F208CD486F9B025CF101E8D0F4DDEC54E24F414C1F1187E302",
            "ftat": "3B3812FC07A431A911A193C5CA1D8A63B184D6E88FA3DAC4CE4DA7D703DBC9C0",
            "onbehalf_id": "FT00087291",
            "eui": "A_9A32677D15A930269279AA9F09FADB263F34859556C0D9901062C9B18318D911",
            "realtime_quote": true,
            "streaming_quote": false,
            "real_time_index_quotes_status": true,
            "real_time_watchlist_quote": true,
            "nls_quote": true
        });

        let response: LoginResponse = serde_json::from_value(json_data).unwrap();
        if let LoginResponse::Verify(verify_response) = response {
            assert_eq!(verify_response.status_code, 200);
            assert_eq!(
                verify_response.sid,
                "87DEF0CF1D54F7F208CD486F9B025CF101E8D0F4DDEC54E24F414C1F1187E302"
            );
            assert_eq!(
                verify_response.ftat,
                "3B3812FC07A431A911A193C5CA1D8A63B184D6E88FA3DAC4CE4DA7D703DBC9C0"
            );
            assert_eq!(verify_response.realtime_quote, true);
        } else {
            panic!("Expected LoginResponse::Verify");
        }
    }
}
