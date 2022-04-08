use yew::prelude::*;
use yew_router::prelude::*;

use bounce::prelude::*;
use bounce::query::{use_mutation_value, QueryStatus};

use wasm_bindgen::{JsCast};

use wasm_bindgen_futures::spawn_local;

use shared_types::types::{UserAuthForm, UserInfo};

use crate::mutations::ServiceError;
use crate::mutations::auth::*;
use crate::stores::auth::*;
use crate::Route;

#[derive(PartialEq)]
struct LoginState {
    is_on_login: bool,
    error: Option<String>,
}

#[function_component(Login)]
pub fn login() -> Html {
    let state = use_state_eq(|| LoginState {
        is_on_login: true,
        error: None
    });

    let auth_mutation = use_mutation_value::<AuthMutation>();
    let credentials = use_atom::<AuthCredentials>();

    let history = use_history();

    let login_class;
    let signup_class;
    let button_text;
    if state.is_on_login {
        login_class = "is-active";
        signup_class = "";
        button_text = "Login"
    } else {
        login_class = "";
        signup_class = "is-active";
        button_text = "Create Account"
    }

    let disabled = if let AuthCredentials::Verified(_) = *credentials {
        true
    } else {
        auth_mutation.status() == QueryStatus::Loading
    };

    let switch_to_login = {
        let state = state.clone();
        Callback::from(move |_| state.set(LoginState {
            is_on_login: true,
            error: None
        }))
    };

    let switch_to_register = {
        let state = state.clone();
        Callback::from(move |_| state.set(LoginState {
            is_on_login: false,
            error: None
        }))
    };

    let submit_form = {
        let state = state.clone();
        Callback::from(move |e: web_sys::FocusEvent| {
            e.prevent_default();

            let data = web_sys::FormData::new_with_form(
                e.target().unwrap().dyn_ref::<web_sys::HtmlFormElement>().unwrap()
            ).unwrap();

            let form = UserAuthForm {
                user_id: data.get("username").as_string().unwrap(),
                password:  data.get("password").as_string().unwrap()
            };

            let login_as = if state.is_on_login {
                LoginAs::RegisteredUser
            } else {
                LoginAs::NewUser
            };

            let auth_mutation = auth_mutation.clone();
            let state = state.clone();
            let history = history.clone();
            let credentials = credentials.clone();
            spawn_local(async move {
                let user_id = form.user_id.clone();
                let result = auth_mutation.run(AuthRequest {
                    data: form,
                    login_as
                }).await;

                match result {
                    Ok(_) => {
                        credentials.set(
                            AuthCredentials::Verified(UserInfo {
                                user_id
                            })
                        );
                        history.unwrap().push(Route::Home)
                    },
                    Err(err) => state.set(
                        LoginState {
                            is_on_login: state.is_on_login,
                            error: Some(match err {
                                AuthError::InvalidCredentials => "Invalid username or password",
                                AuthError::RegisterTakenUsername => "Username already in use",
                                AuthError::Other(ServiceError::UnableToContactServer) => "Unable to contact server, please try again later",
                                AuthError::Other(ServiceError::InternalServerError) => "Server error occured, please try again later"
                            }.into())
                        }
                    )
                }
            });
        })
    };

    let on_form_input = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(LoginState {
                is_on_login: state.is_on_login,
                error: None
            })
        })
    };

    html! {
        <div class="container is-max-desktop center-form">
            <h1 class="title has-text-centered mt-6">{"Boardgames"}</h1>
            <div class="tabs mt-6">
                <ul>
                <li class={login_class} onclick={switch_to_login}>
                    <a>{"Login"}</a>
                </li>
                <li class={signup_class} onclick={switch_to_register}>
                    <a>{"Sign up"}</a>
                </li>
                </ul>
            </div>
            <form onsubmit={submit_form} oninput={on_form_input}>
                <div class="field mt-4">
                    <label class="label">{"Username"}</label>
                    <div class="control">
                        <input class="input" type="username" name="username" placeholder="e.g. lora"/>
                    </div>
                </div>
                <div class="field mt-4">
                    <label class="label">{"Password"}</label>
                    <div class="control">
                        <input class="input" type="password" name="password" placeholder="********"/>
                    </div>
                </div>
                <p class="help is-danger" hidden={state.error.is_none()}>{state.error.as_ref().unwrap_or(&"".into())}</p>

                <button class="button is-primary mt-4" {disabled}>{button_text}</button>
            </form>
        </div>
    }
}