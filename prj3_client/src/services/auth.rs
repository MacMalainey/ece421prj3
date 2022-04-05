use std::rc::Rc;

use async_trait::async_trait;

use wasm_bindgen_futures::spawn_local;

use bounce::prelude::*;
use bounce::query::{Mutation, MutationResult, UseMutationValueHandle};

use shared_types::types::{UserInfo, UserAuthForm};

use super::util::*;

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

#[derive(Debug, PartialEq)]
pub enum AuthCredentials {
    Verified(UserInfo),
    Guest
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LoginAs {
    NewUser,
    RegisteredUser
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum AuthAction {
    UseCredentials(UserAuthForm, LoginAs),
    UseGuest,
    UseCached
}

#[async_trait(?Send)]
impl Mutation for AuthCredentials {
    type Input = AuthAction;
    type Error = AuthError;

    async fn run(_states: &BounceStates, input: Rc<AuthAction>) -> MutationResult<Self> {

        match input.as_ref() {
            AuthAction::UseCredentials(using, las) => {
                use LoginAs::*;
                match las {
                    NewUser => register(using).await,
                    RegisteredUser => login(using).await
                }.map(
                    |_| AuthCredentials::Verified(
                        UserInfo {
                            user_id: using.user_id.clone()
                        }
                    ).into()
                ).map_err(
                    |err| {
                        match err {
                            APIError::AuthenticationError => {
                                match las {
                                    NewUser => AuthError::RegisterTakenUsername,
                                    RegisteredUser => AuthError::InvalidCredentials
                                }
                            }
                            err => AuthError::Other(super::ServiceError::from(err))
                        }
                    }
                )
            },
            AuthAction::UseGuest => {
                logout_client();
                Ok(AuthCredentials::Guest.into())
            },
            AuthAction::UseCached => todo!()
        }

    }
}

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

fn logout_client() {
    wasm_cookies::delete("user_auth_token")
}

pub fn register_user_as(cred: UseMutationValueHandle<AuthCredentials>, user: UserAuthForm) {
    spawn_local(
        async move {
            cred.run(AuthAction::UseCredentials(user, LoginAs::NewUser)).await;
        }
    )
}

pub fn login_using_input(cred: UseMutationValueHandle<AuthCredentials>, user: UserAuthForm) {
    spawn_local(
        async move {
            cred.run(AuthAction::UseCredentials(user, LoginAs::RegisteredUser)).await;
        }
    )
}

pub fn login_using_guest(cred: UseMutationValueHandle<AuthCredentials>) {
    spawn_local(
        async move {
            cred.run(AuthAction::UseGuest).await;
        }
    )
}

pub fn login_using_cached(cred: UseMutationValueHandle<AuthCredentials>) {
    spawn_local(
        async move {
            cred.run(AuthAction::UseCached).await;
        }
    )
}



// async fn logout() -> Result<(), APIError> {
//     let endpoint_url = get_base_url().join("api/v1/user/logout").unwrap();

//     let client = reqwest::Client::new();

//     client.post(endpoint_url)
//         .send()
//         .await?;

//     Ok(())
// }

