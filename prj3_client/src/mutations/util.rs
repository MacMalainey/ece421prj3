use reqwest::Url;

/// Util function for getting the base url of the server backend
pub fn get_base_url() -> Url {
    let origin = web_sys::window().unwrap().location().origin().unwrap();
    Url::parse(&origin).unwrap()
}

/// API Error return types
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

/// Client request builder helper function
pub trait AppendQuery {
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