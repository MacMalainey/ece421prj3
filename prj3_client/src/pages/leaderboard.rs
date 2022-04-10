use yew::prelude::*;

#[derive(PartialEq)]
struct LeaderboardState {
    isOnConnect4: bool,
    error: Option<String>,
}

pub enum Msg {
    SwitchToConnect,
    SwitchToToot
}

/// Login page component
#[function_component(Leaderboard)]
pub fn leaderboard() -> Html {
    // Get state
    let state = use_state_eq(|| LeaderboardState {
        isOnConnect4: true,
        error: None
    });

    let mut connect_class = "is-active";
    let mut toot_class = "";

    if !state.isOnConnect4 {
        connect_class = "";
        toot_class = "is-active";
    }
    // Callback for switching to register
    let switch_to_toot = {
        let state = state.clone();
        Callback::from(move |_| state.set(LeaderboardState {
            isOnConnect4: false,
            error: None
        }))
    };

    let switch_to_connect = {
        let state = state.clone();
        Callback::from(move |_| state.set(LeaderboardState {
            isOnConnect4: true,
            error: None
        }))
    };

    html! {
            <div class="container mt-6" style={"max-width:500px;"}>
                {
                    if state.isOnConnect4 {
                        html! {
                            <h1 class="title has-text-centered mt-6">{"Connect 4 Leaderboard"}</h1>
                        }
                    } else {
                        html! {
                            <h1 class="title has-text-centered mt-6">{"TOOT OTTO Leaderboard"}</h1>
                        }
                    }
                }
                <div class="tabs is-centered is-boxed pt-5">
                    <ul>
                        <li class={connect_class} onclick={switch_to_connect}>
                          <a>
                            <span>{"Connect 4"}</span>
                          </a>
                        </li>
                        <li class={toot_class}  onclick={switch_to_toot}>
                          <a>
                            <span>{"TOOT OTTO"}</span>
                          </a>
                        </li>
                    </ul>
                </div>
                <div class="leaderboard-card">
                    <div class="rank bold">{"Rank"}</div>
                    <div class="name bold">{"Name"}</div>
                    <div class="difficulty bold">{"Difficulty"}</div>
                    <div class="moves bold">{"Moves"}</div>
                </div>
                {
                    if state.isOnConnect4 {
                        html! {
                            <div class="leaderboard-card mt-2 mb-2">
                                <div class="rank">{"#1"}</div>
                                <div class="name">{"Lora"}</div>
                                <div class="difficulty">{"Hard"}</div>
                                <div class="moves">{"2"}</div>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="leaderboard-card mt-2 mb-2">
                                <div class="rank">{"#1"}</div>
                                <div class="name">{"Ben"}</div>
                                <div class="difficulty">{"Easy"}</div>
                                <div class="moves">{"2"}</div>
                            </div>
                        }
                    }
                }
            </div>
        }
}
