use yew::prelude::*;

use bounce::query::{use_mutation_value};

use shared_types::types::{MatchQuerySortBy, MatchQueryFilter, GameType, CpuLevel, MatchResult, Records, MatchRecord};

use crate::mutations::match_records::{MatchRecordQuery, MatchRecordQueryOptions};

#[derive(PartialEq)]
struct LeaderboardState {
    is_on_connect_4: bool,
    error: Option<String>,
}

/// Login page component
#[function_component(Leaderboard)]
pub fn leaderboard() -> Html {
    // Get state
    let state = use_state_eq(|| LeaderboardState {
        is_on_connect_4: true,
        error: None
    });

    let should_refresh_query = use_mut_ref(|| true);

    // Get the option to filter the game
    let game_filter = if state.is_on_connect_4 {
        GameType::Connect4
    } else {
        GameType::OttoToot
    };

    // todo: determine which CPU types to filter

    // Get query handle
    let records_query = use_mutation_value::<MatchRecordQuery>();

    // Handle the query state (doesn't need to be handed here)
    let srq = *should_refresh_query.borrow();
    let body = match records_query.result() {
        Some(Ok(query)) if !srq => {
            let records: &Records<MatchRecord> = &query.0; // Left the type in to make it easy to identify what it is
            let x = &records.records;
            //render each leaderboard record
            x.iter().enumerate().map(|(i, record)| {
                log::info!("{:#?}", record);
                let level;
                if record.cpu_level == CpuLevel::Easy {
                    level = "Easy";
                } else if record.cpu_level == CpuLevel::Medium {
                    level = "Medium";
                } else {
                    level = "Hard";
                }
                html! {
                    <div class="leaderboard-card mt-2 mb-2">
                        <div class="rank">{format!("#{}", i+1)}</div>
                        <div class="name">{record.user_id.as_ref().unwrap()}</div>
                        <div class="difficulty">{level}</div>
                        <div class="moves">{record.moves}</div>
                    </div>
                }
            }).collect::<Html>()
        },
        Some(Err(_err)) if !srq => {html!{ <div class="leaderboard-card mt-2 mb-2">{"Error loading leaderboard"}</div> }},
        _ => {html!{ <div class="leaderboard-card mt-2 mb-2">{"Loading leaderboard..."}</div> }}
    };

    // Update query if necessary
    {
        let mut srq_mut = should_refresh_query.borrow_mut();
        if *srq_mut {
            *srq_mut = false;

            // Set up filter options
            let filters = MatchQueryFilter {
                result: vec![MatchResult::Win],
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

    if !state.is_on_connect_4 {
        connect_class = "";
        toot_class = "is-active";
    }
    // Callback for switching to toot leaderboard
    let switch_to_toot = {
        let state = state.clone();
        let should_refresh_query = should_refresh_query.clone();
        Callback::from(move |_| {
            *should_refresh_query.borrow_mut() = true;
            state.set(LeaderboardState {
                is_on_connect_4: false,
                error: None
            })
        })
    };

    // Callback for switching to connect 4 leaderboard
    let switch_to_connect = {
        let state = state.clone();
        let should_refresh_query = should_refresh_query.clone();
        Callback::from(move |_| {
            *should_refresh_query.borrow_mut() = true;
            state.set(LeaderboardState {
                is_on_connect_4: true,
                error: None
            })
        })
    };

    html! {
            <div class="container mt-6" style={"max-width:500px;"}>
                {
                    if state.is_on_connect_4 {
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
                { body }
            </div>
        }
}
