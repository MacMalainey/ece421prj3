use yew::prelude::*;

use bounce::query::{use_mutation_value};

use shared_types::types::{MatchQuerySortBy, MatchQueryFilter, GameType, CpuLevel, MatchResult, Records, MatchRecord};

use crate::mutations::match_records::{MatchRecordQuery, MatchRecordQueryOptions};

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

    let should_refresh_query = use_mut_ref(|| true);

    // Get the option to filter the game
    let game_filter = if state.isOnConnect4 {
        GameType::Connect4
    } else {
        GameType::OttoToot
    };

    // todo: determine which CPU types to filter

    // Get query handle
    let records_query = use_mutation_value::<MatchRecordQuery>();

    // Handle the query state (doesn't need to be handed here)
    let srq = *should_refresh_query.borrow();
    match records_query.result() {
        Some(Ok(query)) if !srq => {},
        Some(Err(_err)) if !srq => {},
        _ => {}
    }

    // Update query if necessary
    {
        let mut srq_mut = should_refresh_query.borrow_mut();
        if *srq_mut {
            *srq_mut = false;

            // Set up filter options
            let filters = MatchQueryFilter {
                result: vec![MatchResult::Loss],
                game: vec![game_filter],
                level: vec![]
            };

            let records_query = records_query.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _res = records_query.run(
                    MatchRecordQueryOptions {
                        limit: None,                               // Option<i64>
                        offset: None,                              // Option<i64>
                        filters: Some(filters),                    // &Option<MatchQueryFilter>
                        sort_by: Some(MatchQuerySortBy::Duration), // Option<MatchQuerySortBy>
                        asc: Some(true)                            // Option<bool>
                    }
                ).await;
            })
        }
    }

    let mut connect_class = "is-active";
    let mut toot_class = "";

    if !state.isOnConnect4 {
        connect_class = "";
        toot_class = "is-active";
    }
    // Callback for switching to register
    let switch_to_toot = {
        let state = state.clone();
        let should_refresh_query = should_refresh_query.clone();
        Callback::from(move |_| {
            *should_refresh_query.borrow_mut() = true;
            state.set(LeaderboardState {
                isOnConnect4: false,
                error: None
            })
        })
    };

    let switch_to_connect = {
        let state = state.clone();
        let should_refresh_query = should_refresh_query.clone();
        Callback::from(move |_| {
            *should_refresh_query.borrow_mut() = true;
            state.set(LeaderboardState {
                isOnConnect4: true,
                error: None
            })
        })
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
