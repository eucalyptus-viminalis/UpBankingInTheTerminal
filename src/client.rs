use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;

use crate::models::common::{ErrorResponse, PaginatedResponse, SingleResponse};

const BASE_URL: &str = "https://api.up.com.au/api/v1";

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API error ({status}): {detail}")]
    Api { status: String, detail: String },
    #[error("Rate limited — please wait and try again")]
    RateLimited,
}

pub struct UpClient {
    http: reqwest::Client,
}

impl UpClient {
    pub fn new(token: &SecretString) -> Result<Self, ClientError> {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", token.expose_secret());
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).expect("invalid token characters"),
        );

        let http = reqwest::Client::builder()
            .use_rustls_tls()
            .default_headers(headers)
            .build()?;

        Ok(Self { http })
    }

    /// Perform a GET request and deserialize a single-resource response.
    pub async fn get_one<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<SingleResponse<T>, ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.get(&url).send().await?;
        self.handle_response(resp).await
    }

    /// Perform a GET request and deserialize a paginated response.
    pub async fn get_many<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        params: &[(String, String)],
    ) -> Result<PaginatedResponse<T>, ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.get(&url).query(params).send().await?;
        self.handle_response(resp).await
    }

    /// Fetch the next page from a pagination link (absolute URL).
    pub async fn get_next_page<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<PaginatedResponse<T>, ClientError> {
        let resp = self.http.get(url).send().await?;
        self.handle_response(resp).await
    }

    /// Perform a raw GET and return the deserialized body (for ping etc).
    pub async fn get_raw<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T, ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.get(&url).send().await?;
        self.handle_response(resp).await
    }

    /// Perform a POST with a JSON body and return deserialized response.
    pub async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<SingleResponse<T>, ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.post(&url).json(body).send().await?;
        self.handle_response(resp).await
    }

    /// Perform a POST that returns 204 No Content.
    pub async fn post_no_content<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.post(&url).json(body).send().await?;
        self.handle_empty_response(resp).await
    }

    /// Perform a PATCH with a JSON body that returns 204 No Content.
    pub async fn patch_no_content<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.patch(&url).json(body).send().await?;
        self.handle_empty_response(resp).await
    }

    /// Perform a DELETE with a JSON body that returns 204 No Content.
    pub async fn delete_with_body<B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.delete(&url).json(body).send().await?;
        self.handle_empty_response(resp).await
    }

    /// Perform a DELETE that returns 204 No Content.
    pub async fn delete(&self, path: &str) -> Result<(), ClientError> {
        let url = format!("{}{}", BASE_URL, path);
        let resp = self.http.delete(&url).send().await?;
        self.handle_empty_response(resp).await
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        resp: reqwest::Response,
    ) -> Result<T, ClientError> {
        let status = resp.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(ClientError::RateLimited);
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&body) {
                if let Some(err) = err_resp.errors.first() {
                    return Err(ClientError::Api {
                        status: err.status.clone(),
                        detail: err.detail.clone(),
                    });
                }
            }
            return Err(ClientError::Api {
                status: status.to_string(),
                detail: body,
            });
        }

        Ok(resp.json().await?)
    }

    async fn handle_empty_response(&self, resp: reqwest::Response) -> Result<(), ClientError> {
        let status = resp.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(ClientError::RateLimited);
        }

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&body) {
                if let Some(err) = err_resp.errors.first() {
                    return Err(ClientError::Api {
                        status: err.status.clone(),
                        detail: err.detail.clone(),
                    });
                }
            }
            return Err(ClientError::Api {
                status: status.to_string(),
                detail: body,
            });
        }

        Ok(())
    }
}
