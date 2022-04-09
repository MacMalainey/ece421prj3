// Code templated from https://github.com/yewstack/yew/tree/master/examples/router
use yew::prelude::*;
use yew_router::prelude::*;

use bounce::BounceRoot;
use bounce::prelude::*;

mod pages;
mod components;
mod mutations;
mod stores;

use pages::{
    connect_4_setup::Connect4Setup, home::Home, login::Login, page_not_found::PageNotFound, toot_setup::TootSetup
};

use stores::auth::AuthCredentials;

#[derive(Properties, PartialEq)]
struct AppLaunchProps {
    session_user: Option<shared_types::types::UserInfo>
}

#[derive(Properties, PartialEq)]
struct NavBarProps {
    session_user: Option<shared_types::types::UserInfo>
}

/// Routes
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

/// Switch algorithm for [Route]
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

/// Top level component
#[function_component(App)]
fn app(props: &AppLaunchProps) -> Html {
    html! {
        <BrowserRouter>
            <BounceRoot>
                <NavBar session_user={props.session_user.clone()}/>
                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BounceRoot>
        </BrowserRouter>
    }
}

/// Navigation bar component
#[function_component(NavBar)]
fn nav_bar(props: &NavBarProps) -> Html {

    let credentials = use_atom::<AuthCredentials>();

    {
        let credentials = credentials.clone();
        use_effect_with_deps(
            move |session_user| {
                if let Some(user) = session_user {
                    credentials.set(AuthCredentials::Verified(user.clone()))
                };
                || {}
            },
            props.session_user.clone()
        )
    }

    let user = match *credentials {
        AuthCredentials::Verified(ref info) => Some(info.user_id.clone()),
        _ => None,
    };

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
                                Callback::from(move |_| {
                                    let credentials = credentials.clone();
                                    // Log client out
                                    wasm_bindgen_futures::spawn_local(async move {
                                        let res = mutations::auth::logout().await;

                                        match res {
                                            Ok(()) => credentials.set(AuthCredentials::Guest.into()),
                                            Err(err) => {
                                                log::error!("{:#?}", err);
                                                web_sys::window()
                                                    .unwrap()
                                                    .alert_with_message("Error occured, try again later")
                                                    .unwrap()
                                            }
                                        }
                                        
                                    });
                                });
                            html! {
                                <div class="navbar-item has-dropdown is-hoverable">
                                    <div class="navbar-link">
                                        { format!("Hello {}", username) }
                                    </div>
                                    <div class="navbar-dropdown">
                                        <a class="navbar-item" onclick={on_logout}>{"Logout"}</a>
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

/// Entry point
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    wasm_bindgen_futures::spawn_local(async {
        let session_user = mutations::auth::verify().await;

        log::debug!("Session: {:#?}", session_user);
    
        yew::start_app_with_props::<App>(AppLaunchProps{ session_user });
    })
}
