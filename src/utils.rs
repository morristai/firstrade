use crate::error::{Error, ErrorKind, Result};
use crate::models::session::ErrorResponse;
use crate::session::FtCreds;
use crate::url::{ACCESS_TOKEN, USER_AGENT};
use axum::http::HeaderMap;
use http::HeaderValue;
use http::header::InvalidHeaderValue;
use reqwest::{Client as HttpClient, Response};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::time::Duration;

pub(crate) fn build_default_https_client() -> HttpClient {
    let mut headers = HeaderMap::new();
    headers.insert("Connection", "Keep-Alive".parse().unwrap());
    headers.insert("User-Agent", USER_AGENT.parse().unwrap());
    headers.insert("access-token", ACCESS_TOKEN.parse().unwrap());

    HttpClient::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client")
}

#[inline]
pub(crate) fn request_header_error(err: InvalidHeaderValue) -> Error {
    Error::new(ErrorKind::ConfigInvalid, "configuring request headers").with_context("error", err)
}

#[inline]
pub(crate) fn login_credential_error(field: &str) -> Error {
    Error::new(ErrorKind::LoginFailed, format!("missing field: {field}"))
}

#[inline]
pub(crate) fn read_response_error(err: reqwest::Error) -> Error {
    Error::new(ErrorKind::Unexpected, "reading response body").with_context("error", err)
}

#[inline]
pub(crate) fn parse_json_error(err: serde_json::Error) -> Error {
    Error::new(ErrorKind::Unexpected, "parsing response").with_context("response_body", err)
}

pub(crate) async fn handle_failed_response(resp: Response) -> Error {
    let status = resp.status().as_u16();
    let url = resp.url().clone();
    let body = match resp.text().await.map_err(read_response_error) {
        Ok(b) => b,
        Err(err) => return err,
    };

    let mut kind = match status {
        500..=599 => ErrorKind::ServerError,
        401 => ErrorKind::Unauthorized,
        403 => ErrorKind::Forbidden,
        _ => ErrorKind::Unexpected,
    };

    let (message, login_err) = serde_json::from_str::<ErrorResponse>(&body)
        .map(|login_err| (format!("{login_err:?}"), Some(login_err)))
        .unwrap_or_else(|_| (body.to_string(), None));

    if let Some(login_err) = login_err {
        kind = match login_err.error.as_str() {
            "Unauthorized" => ErrorKind::Unauthorized,
            "Forbidden" => ErrorKind::Forbidden,
            _ => ErrorKind::Unexpected,
        };
    }

    let mut err = Error::new(kind, message);
    err = err.with_context("url", format!("{url:?}"));

    err
}

pub(crate) async fn get_with_auth<T: DeserializeOwned>(
    client: &HttpClient,
    url: String,
    cred: &FtCreds,
) -> Result<T> {
    let mut headers = HeaderMap::new();
    headers.insert("ftat", HeaderValue::from_str(cred.ftat.as_str()).unwrap());
    headers.insert("sid", HeaderValue::from_str(cred.sid.as_str()).unwrap());

    let response = client.get(url).headers(headers).send().await.map_err(|e| {
        Error::new(ErrorKind::Unexpected, "Failed to send request").with_context("response", e.to_string())
    })?;

    if !response.status().is_success() {
        return Err(handle_failed_response(response).await);
    }

    let body = response.text().await.map_err(read_response_error)?;
    let data = serde_json::from_str(&body).map_err(parse_json_error)?;
    Ok(data)
}

pub(crate) async fn post_with_auth<T: DeserializeOwned>(
    client: &HttpClient,
    url: String,
    body: &HashMap<&str, &str>,
    cred: &FtCreds,
) -> Result<T> {
    let mut headers = HeaderMap::new();
    headers.insert("ftat", HeaderValue::from_str(cred.ftat.as_str()).unwrap());
    headers.insert("sid", HeaderValue::from_str(cred.sid.as_str()).unwrap());

    let response = client
        .post(url)
        .headers(headers)
        .form(&body)
        .send()
        .await
        .map_err(|e| {
            Error::new(ErrorKind::Unexpected, "Failed to send request")
                .with_context("response", e.to_string())
        })?;

    if !response.status().is_success() {
        return Err(handle_failed_response(response).await);
    }

    let body = response.text().await.map_err(read_response_error)?;
    let data = serde_json::from_str(&body).map_err(parse_json_error)?;
    Ok(data)
}

pub(crate) async fn delete_with_auth<T: DeserializeOwned>(
    client: &HttpClient,
    url: String,
    cred: &FtCreds,
) -> Result<T> {
    let mut headers = HeaderMap::new();
    headers.insert("ftat", HeaderValue::from_str(cred.ftat.as_str()).unwrap());
    headers.insert("sid", HeaderValue::from_str(cred.sid.as_str()).unwrap());

    let response = client.delete(url).headers(headers).send().await.map_err(|e| {
        Error::new(ErrorKind::Unexpected, "Failed to send request").with_context("response", e.to_string())
    })?;

    if !response.status().is_success() {
        return Err(handle_failed_response(response).await);
    }

    let body = response.text().await.map_err(read_response_error)?;
    let data = serde_json::from_str(&body).map_err(parse_json_error)?;
    Ok(data)
}
