mod request;
mod utils;

use anyhow::Result;
use axum::Router;
use axum::routing::{get, post};

use crate::request::*;

use crate::utils::shutdown_signal;
use firstrade::account::FtAccount;
use firstrade::session::{FtSession, FtSessionBuilder, FtSessionConfig};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    ft_client: FtAccount,
}

#[tokio::main]
async fn main() -> Result<()> {
    // NOTE: you can also implement a renew session logic with Account::renew_sid
    let username = std::env::var("FT_USERNAME").expect("FT_USERNAME must be set");
    let password = std::env::var("FT_PASSWORD").expect("FT_PASSWORD must be set");
    let mfa_code = std::env::var("FT_MFA").expect("FT_MFA must be set");

    let mut ft_config = FtSessionConfig::default();
    ft_config.set_username(username);
    ft_config.set_password(password);
    ft_config.set_mfa_code(mfa_code);

    let builder = FtSessionBuilder::new(ft_config)?;
    let mut session = FtSession::from_builder(builder);
    session.login().await?;

    let accounts = session.get_account_list().await?;
    let account_id = accounts
        .items
        .unwrap()
        .first()
        .map(|account| account.account.clone())
        .expect("No accounts found");

    let ft_client = FtAccount::from_session(session, account_id);

    let state = AppState { ft_client };

    let router = Router::new()
        .route("/market-time", get(market_time))
        .route("/user-info", get(user_info))
        .route("/account-list", get(account_list))
        .route("/position", get(account_position))
        .route("/balances", get(account_balances))
        .route("/history", get(account_history))
        .route("/watchlists", get(account_watchlists))
        .route("/watchlists/{:id}", get(watchlist_quote))
        .route("/watchlists/{:id}", post(add_new_watchlist))
        .with_state(state.clone());

    let listener = TcpListener::bind("0.0.0.0:3001").await?;

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server failed to start");

    Ok(())
}
