use yew::prelude::*;
use yew_router::prelude::*;

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

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                //title
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title has-text-centered is-2">{ "Bored?" }</h1>
                        <h2 class="subtitle">{ "We got games!" }</h2>
                    </div>
                </div>

                //render banner
                <div class="tile is-child">
                        <img alt="banner for homepage." src="assets/home_banner1.jpg" />
                </div>

                //render game cards
                <div class="tile is-parent container">
                    { self.game_cards() }
                </div>
            </div>
        }
    }
}
impl Home {
    fn game_cards(&self) -> Html {
        html! {
            <div style={"width:800px; margin-bottom: 80px"}>
                //games title
                <div class="title">{ "Games" }</div>

                //render cards
                <div>
                    //render connect 4 card
                    <div class="tile is-parent" style={"display:inline-block;  width: 300px"}>
                        <div class="tile is-child box" style={"padding-right: 30px; padding-left: 30px;"}>
                            <p class="subtitle has-text-centered">{ "Connect 4" }</p>
                            <img alt="banner for homepage." src="assets/connect.jpg" />
                            <Link<Route> to={Route::Connect4}>
                                <button class="button is-primary my-3" style={"width: 100%;"}>{"Play"}</button>
                            </Link<Route>>
                        </div>
                    </div>

                    //render Toot otto card
                    <div class="tile is-parent" style={"display:inline-block; width: 300px"}>
                        <div class="tile is-child box" style={"padding-right: 30px; padding-left: 30px;"}>
                            <p class="subtitle has-text-centered">{ "TOOT and OTTO" }</p>
                            <img alt="banner for homepage." src="assets/toot.jpg" />
                            <Link<Route> to={Route::TootOtto}>
                                <button class="button is-primary my-3" style={"width: 100%;"}>{"Play"}</button>
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}