use shared_types::types::*;

use reqwest::Url;

fn get_base_url() -> Url {
    let origin = web_sys::window().unwrap().location().origin().unwrap();
    Url::parse(&origin).unwrap()
}

pub async fn get_user_records(limit: Option<i64>, offset: Option<i64>) -> Result<Vec<MatchRecord>, APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/records").unwrap();

    let client = reqwest::Client::new();

    let response = client.get(endpoint_url)
        .query_pair("offset", offset)
        .query_pair("limit", limit)
        .send()
        .await?;

    if !response.status().is_success() {
        Err(APIError::from(response.status()))
    } else {
        let data = response.json::<Vec<MatchRecord>>().await?;
    
        Ok(data)
    }
    
}

pub async fn get_records(
    limit: Option<i64>,
    offset: Option<i64>,
    filters: Option<MatchQueryFilter>,
    sort_by: Option<MatchQuerySortBy>,
    asc: Option<bool>
) -> Result<Vec<MatchRecord>, APIError> {
    let endpoint_url = get_base_url().join("api/v1/games/records").unwrap();

    let client = reqwest::Client::new();

    let response = client.get(endpoint_url)
        .query_pair("offset", offset)
        .query_pair("limit", limit)
        .query_pair("sort_by", sort_by)
        .query(
            &filters
                .map(|f| f.query_pairs())
                .unwrap_or(vec![])
        )
        .query_pair("asc", asc)
        .send()
        .await?
        .json::<Vec<MatchRecord>>()
        .await?;

    Ok(response)
}

pub async fn login(credentials: UserAuthForm) -> Result<(), APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/login").unwrap();

    let client = reqwest::Client::new();

    client.post(endpoint_url)
        .form(&credentials)
        .send()
        .await?;

    Ok(())
}

pub async fn logout() -> Result<(), APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/logout").unwrap();

    let client = reqwest::Client::new();

    client.post(endpoint_url)
        .send()
        .await?;

    Ok(())
}

#[derive(Debug)]
pub enum APIError {
    AuthenticationError,
    InternalServerError,
    TimeoutError,
    RequestError(reqwest::Error),
    UnknownStatus(u16)
}

impl From<reqwest::Error> for APIError {

    fn from(err: reqwest::Error) -> Self {
        use APIError::*;
        if err.is_status() {
            APIError::from(err.status().unwrap())
        } else if err.is_timeout() {
            TimeoutError
        } else {
            RequestError(err)
        }
    }

}

impl From<reqwest::StatusCode> for APIError {

    fn from(status: reqwest::StatusCode) -> Self {
        use APIError::*;
        match status.as_u16() {
            401 => AuthenticationError,
            500 => InternalServerError,
            200..=299 => panic!("Tried to parse http success status code {} as APIError", status),
            other => UnknownStatus(other)
        }
    }

}

trait AppendQuery {
    fn query_pair<T: Into<String>, U: serde::Serialize>(self, label: T, data: Option<U>) -> reqwest::RequestBuilder;
}

impl AppendQuery for reqwest::RequestBuilder {
    fn query_pair<T: Into<String>, U: serde::Serialize>(self, label: T, data: Option<U>) -> reqwest::RequestBuilder {
        if let Some(data) = data {
            self.query(&[(label.into(), data)])
        } else {
            self
        }
    }
}