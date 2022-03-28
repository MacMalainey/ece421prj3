// Code templated from https://github.com/yewstack/yew/tree/master/examples/router
use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::{
    login::Login, home::Home, page_not_found::PageNotFound,
};
use yew::html::Scope;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
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
        Route::Connect4 => html! { <PageNotFound/> },
        Route::TootOtto => html! { <PageNotFound/> },
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
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                    </div>
                </footer>
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
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Board Want Games" }</h1>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>

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
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
