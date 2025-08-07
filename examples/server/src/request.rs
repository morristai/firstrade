use crate::AppState;
use crate::utils::AppError;
use anyhow::{Result, anyhow};
use axum::Json;
use axum::extract::{Path, State};
use firstrade::models::account::{AccountHistory, AccountList, BalanceResult, Position, UserInfo};
use firstrade::models::quote::MarketTimeResponse;
use firstrade::models::watchlist::*;

pub(crate) async fn market_time(State(state): State<AppState>) -> Result<Json<MarketTimeResponse>, AppError> {
    let market_time = state.ft_client.get_market_time().await?;
    Ok(Json(market_time))
}

pub(crate) async fn account_list(State(state): State<AppState>) -> Result<Json<AccountList>, AppError> {
    let account_list = state.ft_client.get_account_list().await?;
    Ok(Json(account_list))
}

pub(crate) async fn user_info(State(state): State<AppState>) -> Result<Json<UserInfo>, AppError> {
    let user_info = state.ft_client.get_user_info().await?;
    Ok(Json(user_info))
}

pub(crate) async fn account_position(State(state): State<AppState>) -> Result<Json<Position>, AppError> {
    let position = state.ft_client.get_account_positions().await?;
    Ok(Json(position))
}

pub(crate) async fn account_balances(State(state): State<AppState>) -> Result<Json<BalanceResult>, AppError> {
    let balances = state.ft_client.get_account_balances().await?;
    let balances = balances
        .result
        .ok_or_else(|| anyhow!("no result found in account balances"))?;

    Ok(Json(balances))
}

pub(crate) async fn account_history(State(state): State<AppState>) -> Result<Json<AccountHistory>, AppError> {
    let history = state
        .ft_client
        .get_account_history("ytd", 1, 200)
        .await?;

    Ok(Json(history))
}

pub(crate) async fn account_watchlists(
    State(state): State<AppState>,
) -> Result<Json<Vec<WatchList>>, AppError> {
    let watchlists = state.ft_client.get_all_watchlists().await?;
    let watchlists: Vec<WatchList> = watchlists
        .items
        .ok_or_else(|| anyhow!("no items found in watchlists"))?;
    Ok(Json(watchlists))
}

pub(crate) async fn watchlist_quote(
    Path(watchlist_id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<WatchListQuoteList>, AppError> {
    let watchlist_quote = state.ft_client.get_watchlist_quote(watchlist_id).await?;
    let watchlist_quote = watchlist_quote
        .result
        .ok_or_else(|| anyhow!("no result found in watchlist quote"))?;

    Ok(Json(watchlist_quote))
}

pub(crate) async fn add_new_watchlist(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<WatchListResponse>, AppError> {
    let response = state.ft_client.add_new_watchlist(name).await?;

    Ok(Json(response))
}
