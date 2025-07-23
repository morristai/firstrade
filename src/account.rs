use crate::error::{Error, ErrorKind, Result};
use crate::models::account::{AccountHistory, AccountList, Balance, Position, UserInfo};
use crate::models::quote::{MarketTime, SingleQuoteResponse};
use crate::models::session::LoginVerifiedResponse;
use crate::models::watchlist::{WatchListQuote, WatchListResponse, WatchLists};
use crate::session::*;
use crate::url::*;
use crate::utils::*;
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use typed_builder::TypedBuilder;

#[derive(Clone, TypedBuilder)]
pub struct FtAccountConfig {
    username: String,
    password: String,
    ftat: String,
    sid: String,
    account_id: String,
    client: Option<HttpClient>,
}

impl From<FtAccountConfig> for FtCreds {
    fn from(builder: FtAccountConfig) -> Self {
        Self {
            username: FirstTradeUsername::from(builder.username),
            password: FirstTradePassword::from(builder.password),
            ftat: FirstTradeAccessToken::from(builder.ftat),
            sid: SessionId::from(builder.sid),
        }
    }
}

#[derive(Clone)]
pub struct FtAccount {
    client: HttpClient,
    account_id: AccountId,
    cred: Arc<RwLock<FtCreds>>,
}

impl FtAccount {
    pub fn new(mut acct_config: FtAccountConfig) -> Self {
        let client = if let Some(client) = acct_config.client.take() {
            client
        } else {
            build_default_https_client()
        };
        let account_id = acct_config.account_id.clone().into();
        let ft_creds: FtCreds = acct_config.into();
        let cred = Arc::new(RwLock::new(ft_creds));

        Self {
            client,
            account_id,
            cred,
        }
    }

    pub fn from_session(session: FtSession, account_id: String) -> Self {
        if let Some(ft_creds) = session.ft_creds {
            let cred = FtCreds {
                username: ft_creds.username,
                password: ft_creds.password,
                ftat: ft_creds.ftat,
                sid: ft_creds.sid,
            };
            let cred = Arc::new(RwLock::new(cred));

            Self {
                client: session.client.clone(),
                account_id: account_id.into(),
                cred,
            }
        } else {
            panic!("Session does not contain FirstTrade credentials");
        }
    }

    pub async fn renew_sid(&self) -> Result<String> {
        let sid;
        {
            let cred = self.cred.read().await;
            let mut body = HashMap::new();
            body.insert("username", cred.username.as_str());
            body.insert("password", cred.password.as_str());
            let resp: LoginVerifiedResponse = post_with_auth(&self.client, login(), &body, &cred).await?;
            sid = resp.sid;
        }
        self.set_new_sid(sid.clone()).await?;
        Ok(sid)
    }

    pub async fn set_new_sid(&self, sid: String) -> Result<()> {
        let mut cred = self.cred.write().await;
        cred.sid = SessionId::from(sid);
        Ok(())
    }

    pub async fn get_sid(&self) -> String {
        let cred = self.cred.read().await;
        cred.sid.as_string()
    }
}

impl FtAccount {
    pub async fn get_market_time(&self) -> Result<MarketTime> {
        let response = self.client.get(market_time()).send().await.map_err(|e| {
            Error::new(ErrorKind::Unexpected, "Failed to send get_market_time request")
                .with_context("response", e.to_string())
        })?;
        if !response.status().is_success() {
            return Err(handle_failed_response(response).await);
        }
        let body = response.text().await.map_err(read_response_error)?;
        let data = serde_json::from_str(&body).map_err(parse_json_error)?;
        Ok(data)
    }

    pub async fn get_account_list(&self) -> Result<AccountList> {
        let url = account_list();
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn get_user_info(&self) -> Result<UserInfo> {
        let url = user_info(self.account_id.as_str());
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn get_account_positions(&self) -> Result<Position> {
        let url = account_positions(self.account_id.as_str());
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn get_account_balances(&self) -> Result<Balance> {
        let url = account_balances(self.account_id.as_str());
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn get_account_history(&self, range: &str, page: u32, per_page: u32) -> Result<AccountHistory> {
        let url = account_history(self.account_id.as_str(), range, page, per_page);
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn get_all_watchlists(&self) -> Result<WatchLists> {
        let url = watchlists();
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn get_watchlist_quote(&self, id: u32) -> Result<WatchListQuote> {
        let url = watchlist_quote(id);
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }

    pub async fn add_new_watchlist(&self, name: String) -> Result<WatchListResponse> {
        let url = watchlists();
        let cred = self.cred.read().await;
        let body = HashMap::from([("name", name.as_str())]);
        post_with_auth(&self.client, url, &body, &cred).await
    }

    pub async fn watchlist_add_symbol(
        &self,
        watchlist_id: u32,
        symbol: String,
        sec_type: u8,
    ) -> Result<WatchListResponse> {
        let url = format!("{}/{}", watchlist(), watchlist_id);
        let cred = self.cred.read().await;

        let mut body = HashMap::new();
        let sec_type = sec_type.to_string();
        body.insert("symbol", symbol.as_str());
        body.insert("sec_type", sec_type.as_str());

        post_with_auth(&self.client, url, &body, &cred).await
    }

    pub async fn watchlist_remove_symbol(&self, symbol_id: u32) -> Result<WatchListResponse> {
        let url = format!("{}/{}", watchlist(), symbol_id);
        let cred = self.cred.read().await;
        delete_with_auth(&self.client, url, &cred).await
    }

    pub async fn delete_watchlist(&self, watchlist_id: u32) -> Result<WatchListResponse> {
        let url = format!("{}/{}", watchlists(), watchlist_id);
        let cred = self.cred.read().await;
        delete_with_auth(&self.client, url, &cred).await
    }

    pub async fn single_quote(&self, symbol: String) -> Result<SingleQuoteResponse> {
        let url = single_quote(self.account_id.as_str(), symbol.as_str());
        let cred = self.cred.read().await;
        get_with_auth(&self.client, url, &cred).await
    }
}
