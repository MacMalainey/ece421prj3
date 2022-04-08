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

/// State for login page
#[derive(PartialEq)]
struct LoginState {
    is_on_login: bool,
    error: Option<String>,
}

/// Login page component
#[function_component(Login)]
pub fn login() -> Html {
    // Get state
    let state = use_state_eq(|| LoginState {
        is_on_login: true,
        error: None
    });

    // Mutation for authentication
    let auth_mutation = use_mutation_value::<AuthMutation>();
    // Credentials we have gotten
    let credentials = use_atom::<AuthCredentials>();

    // For redirect when complete
    let history = use_history().unwrap();

    // CSS stuff
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

    // Disable if we are running the mutation or if we are already logged in
    let disabled = if let AuthCredentials::Verified(_) = *credentials {
        history.push(Route::Home); // If we are logged in we need to re-route
        true
    } else {
        auth_mutation.status() == QueryStatus::Loading
    };

    // Callback for switching to login
    let switch_to_login = {
        let state = state.clone();
        Callback::from(move |_| state.set(LoginState {
            is_on_login: true,
            error: None
        }))
    };

    // Callback for switching to register
    let switch_to_register = {
        let state = state.clone();
        Callback::from(move |_| state.set(LoginState {
            is_on_login: false,
            error: None
        }))
    };

    // Callback when form is to be submitted
    let submit_form = {
        let state = state.clone();
        Callback::from(move |e: web_sys::FocusEvent| {
            e.prevent_default();

            // Get form data
            let data = web_sys::FormData::new_with_form(
                e.target().unwrap().dyn_ref::<web_sys::HtmlFormElement>().unwrap()
            ).unwrap();

            // Convert to our form type
            let form = UserAuthForm {
                user_id: data.get("username").as_string().unwrap(),
                password:  data.get("password").as_string().unwrap()
            };

            // Get login type (login or register)
            let login_as = if state.is_on_login {
                LoginAs::RegisteredUser
            } else {
                LoginAs::NewUser
            };

            // Register the mutation to be ran when ready
            let auth_mutation = auth_mutation.clone();
            let state = state.clone();
            let history = history.clone();
            let credentials = credentials.clone();
            spawn_local(async move {
                // Run mutation
                let user_id = form.user_id.clone();
                let result = auth_mutation.run(AuthRequest {
                    data: form,
                    login_as
                }).await;

                // Parse result
                match result {
                    // Update credentials store (sadly we cannot update stores from a mutation)
                    Ok(_) => {
                        credentials.set(
                            AuthCredentials::Verified(UserInfo {
                                user_id
                            })
                        );
                        history.push(Route::Home)
                    },
                    // Handle error
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

    // Reset errors if the form gets any input
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