use crate::error::{Error, ErrorKind, Result};
use crate::models::account::AccountList;
use crate::models::session::{LoginMfaRequest, LoginResponse};
use crate::url::{ACCESS_TOKEN, account_list, login, verify_pin};
use crate::utils::*;
use async_recursion::async_recursion;
use axum::http::{HeaderMap, HeaderValue};
use derive_more::From;
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::fmt::Debug;
use typed_builder::TypedBuilder;
use zeroize::Zeroize;

#[derive(Clone, Debug, From)]
pub(crate) struct FirstTradeUsername(String);

impl FirstTradeUsername {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, From, Zeroize)]
pub(crate) struct FirstTradePassword(String);

impl FirstTradePassword {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, From, Zeroize)]
pub(crate) struct FirstTradeAccessToken(String);

impl FirstTradeAccessToken {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, From, Zeroize)]
pub(crate) struct SessionId(String);

impl SessionId {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn as_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, Debug, From)]
pub(crate) struct AccountId(String);

impl AccountId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct FtCreds {
    pub(crate) username: FirstTradeUsername,
    pub(crate) password: FirstTradePassword,
    pub(crate) ftat: FirstTradeAccessToken,
    pub(crate) sid: SessionId,
}

impl Debug for FtCreds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FtCreds")
            .field("ftat", &"***")
            .field("sid", &"***")
            .field("access_token", &"***")
            .finish()
    }
}

impl FtCreds {
    pub fn get_ftat(&self) -> String {
        self.ftat.0.clone()
    }

    pub fn get_sid(&self) -> String {
        self.sid.0.clone()
    }
}

// TODO: from Map
#[derive(Clone, TypedBuilder)]
pub struct FtSessionConfig {
    log_level: log::Level,
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    pin: Option<String>,
    mfa_code: Option<String>,
    ftat: Option<String>,
    client: Option<HttpClient>,
}

impl Debug for FtSessionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FtSessionConfig")
            .field("log_level", &self.log_level)
            .field("username", &self.username.as_ref().map(|_| "***"))
            .field("password", &self.password.as_ref().map(|_| "***"))
            .field("pin", &self.pin.as_ref().map(|_| "***"))
            .field("email", &self.email.as_ref().map(|_| "***"))
            .field("phone", &self.phone.as_ref().map(|_| "***"))
            .field("mfa_secret", &self.mfa_code.as_ref().map(|_| "***"))
            .field("access_token", &self.ftat.as_ref().map(|_| "***"))
            .finish()
    }
}

impl Default for FtSessionConfig {
    fn default() -> Self {
        Self {
            log_level: log::Level::Info,
            username: None,
            password: None,
            email: None,
            phone: None,
            pin: None,
            mfa_code: None,
            ftat: None,
            client: None,
        }
    }
}

impl FtSessionConfig {
    pub fn set_username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn set_password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }
    pub fn set_ftat(&mut self, ftat: String) -> &mut Self {
        self.ftat = Some(ftat);
        self
    }

    pub fn set_mfa_code(&mut self, mfa_code: String) -> &mut Self {
        self.mfa_code = Some(mfa_code);
        self
    }
}

#[derive(Debug, Clone)]
pub struct FtSessionBuilder {
    client: HttpClient,
    ft_config: FtSessionConfig,
}

#[derive(Debug, Clone)]
pub struct FtSession {
    pub(crate) client: HttpClient,
    pub(crate) ft_config: FtSessionConfig,
    pub(crate) ft_creds: Option<FtCreds>,
}

impl FtSessionBuilder {
    pub fn new(mut ft_config: FtSessionConfig) -> Result<Self> {
        let client = if let Some(client) = ft_config.client.take() {
            client
        } else {
            build_default_https_client()
        };

        if ft_config.username.is_none() || ft_config.password.is_none() {
            return Err(Error::new(
                ErrorKind::ConfigInvalid,
                "Username and password must be provided for login",
            ));
        }

        if ft_config.ftat.is_none() && ft_config.mfa_code.is_none() {
            return Err(Error::new(
                ErrorKind::ConfigInvalid,
                "Either ftat or mfa_code must be provided",
            ));
        }

        Ok(Self { client, ft_config })
    }
}

impl FtSession {
    pub fn from_builder(builder: FtSessionBuilder) -> Self {
        Self {
            client: builder.client,
            ft_config: builder.ft_config,
            ft_creds: None,
        }
    }

    pub async fn init_login(&mut self) -> Result<()> {
        let mut headers = HeaderMap::new();
        if let Some(ftat) = &self.ft_config.ftat {
            headers.insert("ftat", HeaderValue::from_str(ftat).unwrap());
        }

        let mut body = HashMap::new();
        body.insert("username", &self.ft_config.username);
        body.insert("password", &self.ft_config.password);

        let response = self
            .client
            .post(login())
            .headers(headers)
            .form(&body)
            .send()
            .await
            .map_err(|e| {
                Error::new(ErrorKind::Unexpected, "Failed to send login request")
                    .with_context("response", e.to_string())
            })?;

        if !response.status().is_success() {
            return Err(handle_failed_response(response).await);
        }

        let body = response.text().await.map_err(read_response_error)?;
        let data: LoginResponse = serde_json::from_str(&body).map_err(parse_json_error)?;
        Self::login_verify(self, data).await?;
        Ok(())
    }

    #[async_recursion]
    async fn login_verify(&mut self, resp: LoginResponse) -> Result<()> {
        match resp {
            LoginResponse::Otp(_data) => {
                unimplemented!()
            }
            LoginResponse::Mfa(data) => {
                let t_token = data.t_token;
                let mfa_code = self
                    .ft_config
                    .mfa_code
                    .clone()
                    .ok_or(login_credential_error("mfa_code"))?;

                let body = LoginMfaRequest::builder()
                    .t_token(t_token)
                    .mfa_code(mfa_code)
                    .remember_for(30)
                    .build();
                let body = serde_json::to_string(&body).unwrap();

                let response = self
                    .client
                    .post(verify_pin())
                    .body(body)
                    .send()
                    .await
                    .map_err(|e| {
                        Error::new(ErrorKind::Unexpected, "Failed to send login request")
                            .with_context("response", e.to_string())
                    })?;

                if !response.status().is_success() {
                    return Err(handle_failed_response(response).await);
                }
                let body = response.text().await.map_err(read_response_error)?;
                let data: LoginResponse = serde_json::from_str(&body).map_err(parse_json_error)?;
                Ok(Self::login_verify(self, data).await?)
            }
            LoginResponse::Verify(data) => {
                let username = self.ft_config.username.clone().unwrap();
                let password = self.ft_config.password.clone().unwrap();
                let ft_cred = FtCreds {
                    username: FirstTradeUsername(username),
                    password: FirstTradePassword(password),
                    ftat: FirstTradeAccessToken(data.ftat),
                    sid: SessionId(data.sid),
                };
                self.set_ft_creds(ft_cred);
                Ok(())
            }
        }
    }

    pub fn set_ft_creds(&mut self, creds: FtCreds) {
        self.ft_creds = Some(creds);
    }

    pub fn get_secrets(&self) -> Option<FtCreds> {
        self.ft_creds.clone()
    }

    pub async fn get_account_list(&self) -> Result<AccountList> {
        if let Some(ft_creds) = &self.ft_creds {
            let mut headers = HeaderMap::new();
            headers.insert(
                "ftat",
                HeaderValue::from_str(&ft_creds.ftat.0).map_err(request_header_error)?,
            );
            headers.insert(
                "sid",
                HeaderValue::from_str(&ft_creds.sid.0).map_err(request_header_error)?,
            );
            headers.insert(
                "access-token",
                HeaderValue::from_str(ACCESS_TOKEN).map_err(request_header_error)?,
            );

            let response = self
                .client
                .get(account_list())
                .headers(headers)
                .send()
                .await
                .map_err(|e| {
                    Error::new(ErrorKind::Unexpected, "Failed to send get_account_list request")
                        .with_context("response", e.to_string())
                })?;

            if !response.status().is_success() {
                return Err(handle_failed_response(response).await);
            }
            let body = response.text().await.map_err(read_response_error)?;
            let data: AccountList = serde_json::from_str(&body).map_err(parse_json_error)?;

            Ok(data)
        } else {
            Err(login_credential_error("ft_creds"))
        }
    }
}
