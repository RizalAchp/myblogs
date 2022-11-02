use anyhow::{anyhow, Result};
use reqwest;
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
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            data.json::<T>().await.map_err(|e| anyhow!(e))
        } else {
            match data.status().as_u16() {
                401 => Err(anyhow!("UnAutorized")),
                403 => Err(anyhow!("Forbidden")),
                404 => Err(anyhow!("NotFound")),
                500 => Err(anyhow!("InternalServerError")),
                422 => {
                    let data: std::result::Result<String, _> = data.json().await;
                    if let Ok(data) = data {
                        Err(anyhow!(data))
                    } else {
                        Err(anyhow!("Deserialize error"))
                    }
                }
                _ => Err(anyhow!("Request Error")),
            }
        }
    } else {
        Err(anyhow!("Request Error"))
    }
}

/// Delete request
pub async fn request_delete<T>(url: String) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(reqwest::Method::GET, url, ()).await
}

pub async fn request_get_full<T>(url: String) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT, url);
    request(reqwest::Method::PUT, url, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
