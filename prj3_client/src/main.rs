// Code templated from https://github.com/yewstack/yew/tree/master/examples/router
use yew::prelude::*;
use yew_router::prelude::*;

use bounce::BounceRoot;
use bounce::query::{use_mutation_value};

use crate::services::auth::*;

mod pages;
mod components;
mod services;

use pages::{
    connect_4_setup::Connect4Setup, home::Home, login::Login, page_not_found::PageNotFound, toot_setup::TootSetup
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/games/connect4")]
    Connect4,
    #[at("/games/toototto")]
    TootOtto,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Login => html! { <Login/> },
        Route::Leaderboard => html! { <PageNotFound/> },
        Route::Connect4 => html! { <Connect4Setup/> },
        Route::TootOtto => html! { <TootSetup/> },
        Route::NotFound => html! { <PageNotFound/> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <BounceRoot>
                <NavBar/>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BounceRoot>
        </BrowserRouter>
    }
}

#[function_component(NavBar)]
fn nav_bar() -> Html {

    let credentials = use_mutation_value::<AuthCredentials>();

    let user = match credentials.result() {
        Some(Ok(cred)) => match cred.as_ref() {
            AuthCredentials::Verified(info) => Some(info.user_id.clone()),
            _ => None,
        }
        _ => None
    };

    log::debug!("{:?}", user);

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand ml-3">
                <Link<Route> to={Route::Home}>
                    <img width="45" class="navbar-item" alt="icon" src="assets/main_icon.svg" />
                </Link<Route>>
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <div class="navbar-item has-dropdown is-hoverable">
                        <div class="navbar-link">
                            { "Games" }
                        </div>
                        <div class="navbar-dropdown">
                            <Link<Route> classes={classes!("navbar-item")} to={Route::Connect4}>
                                { "Connect 4" }
                            </Link<Route>>
                            <Link<Route> classes={classes!("navbar-item")} to={Route::TootOtto}>
                                { "TOOT and OTTO " }
                            </Link<Route>>
                        </div>
                    </div>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Leaderboard}>
                        { "Leaderboard" }
                    </Link<Route>>
                </div>
                <div class="navbar-end">
                    {
                        if let Some(username) = user {
                            let on_logout = 
                                Callback::from(move |_| login_using_guest(credentials.clone()));
                            html! {
                                <div class="navbar-item has-dropdown is-hoverable">
                                    <div class="navbar-link">
                                        { format!("Hello {}", username) }
                                    </div>
                                    <div class="navbar-dropdown">
                                        <div class="navbar-item" onclick={on_logout}></div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                                    { "Login" }
                                </Link<Route>>
                            }
                        }
                    }
                    
                </div>
            </div>
        </nav>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}
