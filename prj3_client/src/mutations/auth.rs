use std::rc::Rc;

use async_trait::async_trait;

use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult};

use shared_types::types::{UserAuthForm, UserInfo};

use super::util::*;

/// Error type for Auth APIs
#[derive(Debug, PartialEq, Clone)]
pub enum AuthError {
    InvalidCredentials,
    RegisterTakenUsername,
    Other(super::ServiceError)
}

impl std::fmt::Display for AuthError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            AuthError::InvalidCredentials => write!(f, "Authorization Error: Invalid credentials"),
            AuthError::RegisterTakenUsername => write!(f, "Authorization Error: Attempted to register using taken username"),
            AuthError::Other(err) => write!(f, "Authorization Error: API error ({})", err)
        }
    }

}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AuthError::Other(ref err) => Some(err),
            _ => None
        }
    }
}

/// Specification to login as a new or registered user
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LoginAs {
    NewUser,
    RegisteredUser
}

/// Auth request data
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct AuthRequest {
    pub data: UserAuthForm,
    pub login_as: LoginAs,
}

/// Mutation for logging in, registering, or logging out a user
#[derive(Debug, PartialEq)]
pub struct AuthMutation();

#[async_trait(?Send)]
impl Mutation for AuthMutation {
    type Input = AuthRequest;
    type Error = AuthError;

    async fn run(_states: &BounceStates, input: Rc<AuthRequest>) -> MutationResult<Self> {

        use LoginAs::*;
        match input.login_as {
            NewUser => register(&input.data).await,
            RegisteredUser => login(&input.data).await
        }.map(
            |_| AuthMutation().into()
        ).map_err(
            |err| {
                match err {
                    APIError::AuthenticationError => {
                        match input.login_as {
                            NewUser => AuthError::RegisterTakenUsername,
                            RegisteredUser => AuthError::InvalidCredentials
                        }
                    }
                    err => AuthError::Other(super::ServiceError::from(err))
                }
            }
        )
    }

}

/// Run API call for registering user
async fn register(credentials: &UserAuthForm) -> Result<(), APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/register").unwrap();

    let client = reqwest::Client::new();

    client.post(endpoint_url)
        .form(credentials)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// Run API call for logging in user
async fn login(credentials: &UserAuthForm) -> Result<(), APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/login").unwrap();

    let client = reqwest::Client::new();

    client.post(endpoint_url)
        .form(credentials)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

/// Run API call for verifying cached auth token
pub async fn verify() -> Option<UserInfo> {
    let endpoint_url = get_base_url().join("api/v1/user/verify").unwrap();

    reqwest::get(endpoint_url)
        .await
        .and_then(|resp| resp.error_for_status())
        .ok()?
        .json::<UserInfo>()
        .await
        .ok()
}

pub async fn logout() -> Result<(), APIError> {
    let endpoint_url = get_base_url().join("api/v1/user/logout").unwrap();

    let client = reqwest::Client::new();

    client.post(endpoint_url)
        .send()
        .await?;

    Ok(())
}

