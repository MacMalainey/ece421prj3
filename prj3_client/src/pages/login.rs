use yew::prelude::*;
use yew_router::prelude::*;

use bounce::query::{use_mutation_value};

use wasm_bindgen::{JsCast};

use shared_types::types::UserAuthForm;

use crate::services::ServiceError;
use crate::services::auth::*;
use crate::Route;

#[derive(PartialEq)]
struct LoginState {
    is_on_login: bool,
    has_changed: bool
}

#[function_component(Login)]
pub fn login() -> Html {
    let state = use_state_eq(|| LoginState {
        is_on_login: true,
        has_changed: true
    });

    let credentials = use_mutation_value::<AuthCredentials>();

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

    let mut disabled = false;
    let mut error = None;

    match credentials.result() {
        None => {
            // disabled = true;
        },
        Some(Ok(u)) => {
            log::debug!("{:?}", u);
            history.unwrap().push(Route::Home)
        },
        Some(Err(err)) => if !state.has_changed {
            error = Some(match err {
                AuthError::InvalidCredentials => "Invalid username or password",
                AuthError::RegisterTakenUsername => "Username already in use",
                AuthError::Other(ServiceError::UnableToContactServer) => "Unable to contact server, please try again later",
                AuthError::Other(ServiceError::InternalServerError) => "Server error occured, please try again later"
            })
        }
    }

    let switch_to_login = {
        let state = state.clone();
        Callback::from(move |_| state.set(LoginState {
            is_on_login: true,
            has_changed: true
        }))
    };

    let switch_to_register = {
        let state = state.clone();
        Callback::from(move |_| state.set(LoginState {
            is_on_login: false,
            has_changed: true
        }))
    };

    let submit_form = {
        let state = state.clone();
        let credentials = credentials.clone();
        Callback::from(move |e: web_sys::FocusEvent| {
            e.prevent_default();

            let data = web_sys::FormData::new_with_form(
                e.target().unwrap().dyn_ref::<web_sys::HtmlFormElement>().unwrap()
            ).unwrap();

            let form = UserAuthForm {
                user_id: data.get("username").as_string().unwrap(),
                password:  data.get("password").as_string().unwrap()
            };

            let credentials = credentials.clone();
            if state.is_on_login {
                login_using_input(credentials, form);
            } else {
                register_user_as(credentials, form);
            }

            state.set(LoginState {
                is_on_login: state.is_on_login,
                has_changed: false
            })
        })
    };

    let on_form_input = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(LoginState {
                is_on_login: state.is_on_login,
                has_changed: true
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
                <p class="help is-danger" hidden={error.is_none()}>{error.unwrap_or("")}</p>

                <button class="button is-primary mt-4" {disabled}>{button_text}</button>
            </form>
        </div>
    }
}