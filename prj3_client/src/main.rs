// Code templated from https://github.com/yewstack/yew/tree/master/examples/router
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod components;
mod apis;

use pages::{
    connect_4_setup::Connect4Setup, home::Home, login::Login, page_not_found::PageNotFound, toot_setup:: TootSetup
};
use yew::html::Scope;

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

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, _link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand ml-3">
                  <Link<Route> to={Route::Home}>
                    <img width="45" class="navbar-item" alt="icon" src="assets/main_icon.svg" />
                    </Link<Route>>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
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
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                            { "Login" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}
