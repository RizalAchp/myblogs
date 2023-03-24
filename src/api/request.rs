use anyhow::{anyhow, Result};
use reqwest::{self, Method};
use serde::{de::DeserializeOwned, Serialize};

pub const KEY_REPO: &str = "rizalachp.data.repo";
pub const KEY_PROFILE: &str = "rizalachp.data.profile";
pub const API_ROOT: &str = "https://api.github.com/";
pub const USERNAME_GITHUB: &str = "users/RizalAchp";
pub const API_URL_THIS: &str = "https://api.github.com/repos/{}/myblogs";
/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = (method == Method::POST) || (method == Method::PUT);
    let mut builder = reqwest::Client::new()
        .request(method.to_owned(), &url)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }
    let response = builder
        .send()
        .await
        .map_err(|_| anyhow!("Request error on {url}"))?;

    let status = response.status();
    if status.is_success() {
        response.json::<T>().await.map_err(|e| anyhow!(e))
    } else {
        match status.as_u16() {
            401 => Err(anyhow!("UnAutorized on Request {method} - {url}")),
            403 => Err(anyhow!("Forbidden on Request {method} - {url}")),
            404 => Err(anyhow!("NotFound on Request {method} - {url}")),
            500 => Err(anyhow!("InternalServerError on Request {method} - {url}")),
            422 => {
                let data: std::result::Result<String, _> = response.json().await;
                if let Ok(data) = data {
                    Err(anyhow!(data))
                } else {
                    Err(anyhow!("Deserialize error on Request {method} - {url}"))
                }
            }
            _ => Err(anyhow!("Request Error on Request {method} - {url}")),
        }
    }
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(Method::GET, url, ()).await
}

pub async fn request_get_full<T>(url: String) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(Method::PUT, url, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
