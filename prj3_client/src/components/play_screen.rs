use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;
use bounce::prelude::*;
use bounce::query::{use_mutation_value, MutationResult, UseMutationValueHandle};

use shared_types::types::{MatchResult, CpuLevel, ClientMatchData, GameType};

use crate::game;
use crate::game::*;

use crate::stores::auth::AuthCredentials;
use crate::mutations::match_records::UserMatchRecordMutation;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub selected_difficulty: String,
    pub selected_disc_color: String,
    pub selected_board_size: String,
    pub columns: String,
    pub rows: String,
}

struct PlayScreenState {
    pub game_state: GameState,
    pub board_state: Vec<(i32, String)>,
}

struct BoardUpdateCallbackFactory {
    state: UseStateHandle<PlayScreenState>,
    game: Rc<RefCell<Game>>,
    record_mutation: UseMutationValueHandle<UserMatchRecordMutation>,
    game_type: GameType,
    cpu_level: CpuLevel,
    is_guest: bool,
}

impl BoardUpdateCallbackFactory {

    fn is_game_finished(&self) -> bool {
        self.game.borrow().get_state() != GameState::Running
    }

    fn get_callback_for<I>(&self, i: usize) -> Callback<I> {
        let game = self.game.clone();
        let state = self.state.clone();
        let record_mutation = self.record_mutation.clone();
        let game_id = self.game_type;
        let cpu_level = self.cpu_level;
        let is_guest = self.is_guest;
        Callback::from(move |_| {
            let mut game_mut = game.borrow_mut();
            if game_mut.player_turn(i) {
                // Check for victory/tie
                let mut game_state = game_mut.check_state(game::PLAYER_ID);

                if game_state == GameState::Running {
                    // Perform AI turn
                    game_mut.ai_turn();

                    // Check for loss/tie
                    game_state = game_mut.check_state(game::AI_ID);
                }

                if game_state != GameState::Running && !is_guest {
                    let result = if game_state == GameState::Win(game::PLAYER_ID) {
                        MatchResult::Win
                    } else if game_state == GameState::Win(game::AI_ID) {
                        MatchResult::Loss
                    } else if game_state == GameState::Tie {
                        MatchResult::Tie
                    } else {
                        panic!("Unknown Game State")
                    };

                    let moves = game_mut.get_num_moves() as i32;

                    let record_mutation = record_mutation.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let _res = record_mutation.run(ClientMatchData {
                            game_id,
                            cpu_level,
                            moves: moves / 2 + moves % 2,
                            result
                        }).await;
                    });
                }

                let board_state = game_mut.get_board_state();

                // It appears that state.set() runs synchronously which means that the mutable reference is still active so we drop it here
                std::mem::drop(game_mut);
                state.set(PlayScreenState { board_state, game_state }.into())
            }
        })
    }
}

#[function_component(PlayScreen)]
pub fn play_screen(props: &Props) -> Html {
    let name = props.name.clone();
    let selected_color = props.selected_disc_color.clone();
    let mode = props.selected_difficulty.clone();

    let (ai_config, cpu_level) = get_ai_config(&mode);

    let game = use_mut_ref(|| {
        Game::new(
            props.rows.clone().parse::<usize>().unwrap(),
            props.columns.clone().parse::<usize>().unwrap(),
            ai_config
        )
    });

    let state = {
        let game = game.clone();
        use_state(move || {
            PlayScreenState {
                game_state: GameState::Running,
                board_state: game.borrow().get_board_state(),
            }
        })
    };

    let user = use_atom_value::<AuthCredentials>();

    let record_mutation = use_mutation_value::<UserMatchRecordMutation>();

    let (p1, p2, game_type) = if name == "TOOT and OTTO" {
        ("You - TOOT", "Computer - OTTO", GameType::OttoToot)
    } else {
        ("You", "Computer", GameType::Connect4)
    };

    let is_guest = *user == AuthCredentials::Guest;

    let cb_factory = BoardUpdateCallbackFactory {
        state: state.clone(),
        game: game.clone(),
        record_mutation: record_mutation.clone(),
        game_type,
        cpu_level,
        is_guest
    };

    html! {
        <div class="container" style="max-width:650px">
            <h1 class="title has-text-centered mt-6">{name}</h1>
            <div class="mt-6">
                <div class="in-game-player-info">
                    <div style={"height: 15px; width: 15px; border-radius: 50%; background-color:".to_string() + &selected_color.to_string()}/>
                    <div style={""}>{p1}</div>
                </div>
                <div class="in-game-player-info">
                    <div style={"height: 15px; width: 15px; border-radius: 50%; background-color:".to_string() + get_opponent_color(selected_color.to_string())}/>
                    <div style={""}>{p2}</div>
                </div>
                <div style={"float:right"}>{format!("{} mode", mode)}</div>
            </div>
            <div class="card mt-2">
                {
                    render_grid(
                        props.selected_board_size.clone(),
                        state.board_state.clone(),
                        props.selected_disc_color.clone()
                    )
                }
                {
                    if state.game_state == GameState::Running {
                        render_col_buttons(
                            cb_factory,
                            props.selected_board_size.clone(),
                        )
                    } else {
                        let result = get_result_text(state.game_state).to_string();

                        let on_restart_clicked = {
                            let props = props.clone();
                            Callback::from(move |_| {
                                *(game.borrow_mut()) = Game::new(
                                    props.rows.clone().parse::<usize>().unwrap(),
                                    props.columns.clone().parse::<usize>().unwrap(),
                                    ai_config
                                );
                                state.set(PlayScreenState {
                                    board_state: game.borrow().get_board_state(),
                                    game_state: GameState::Running,
                                }.into());
                            })
                        };

                        html! {
                            <div class="card results-card">
                                <div class="card-content">
                                <div class="content">
                                    <h1 class="title has-text-centered">{result}</h1>
                                    <button class="button is-primary block" onclick={on_restart_clicked} style={"width: 100%;"}>{"Play again"}</button>
                                    {render_record_save(record_mutation.result(), is_guest)}
                                </div>
                                </div>
                            </div>
                        }
                    }
                }
            </div>
        </div>
    }
}

fn get_ai_config(diff: &str) -> (AIConfiguration, CpuLevel) {
    match diff {
        "Easy" => {
            (game::AI_EASY, CpuLevel::Easy)
        },
        "Medium" => {
            (game::AI_MEDIUM, CpuLevel::Medium)
        },
        "Hard" => {
            (game::AI_HARD, CpuLevel::Hard)
        },
        _ => {
            (game::AI_EASY, CpuLevel::Easy)
        }
    }
}

fn get_result_text(state: GameState) -> &'static str {
    if state == GameState::Tie {
        "You tied"
    } else if state == GameState::Win(PLAYER_ID) {
        "You win!"
    } else {
        "You lost :("
    }
}

fn get_opponent_color(selected_disc_color: String) -> &'static str {
    if selected_disc_color == "#FF8E8E" {
        "#FFE68E"
    } else {
        "#FF8E8E"
    }
}

fn render_grid(selected_board_size: String, board_state: Vec<(i32, String)>, selected_disc_color: String) -> Html {
    let split: Vec<&str> = selected_board_size.split("x").collect();
    let cols = split[0];
    // let rows = split[1];
    html! {
        <>
            <div class={"background-3 grid-container grid_cols_".to_string() + &cols.to_string()}>
                {
                    board_state.into_iter().map(|(piece, letter)| {
                        if piece == 1 {
                            html!{
                                <div class="grid-item">
                                    <div class="circle" style={"background-color:".to_string() + &selected_disc_color.to_string()}>
                                        <div>{letter}</div>
                                    </div>
                                </div>
                            }
                        }
                        else if piece == 2 {
                            html! {
                                <div class="grid-item">
                                    <div class="circle" style={"background-color:".to_string() + get_opponent_color(selected_disc_color.to_string())}>
                                        <div>{letter}</div>
                                    </div>
                                </div>
                            }
                        } else {
                                html!{
                                <div class="grid-item">
                                    <div class="circle" style={"background-color: white"}/>
                                </div>
                            }
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}

fn render_col_buttons(
    cb_factory: BoardUpdateCallbackFactory,
    selected_board_size: String
) -> Html {
    let split: Vec<&str> = selected_board_size.split("x").collect();
    let cols = split[0];
    html! {
        <>
            <div class={"col-button-container grid-container grid_cols_".to_string() + &cols.to_string()}>
                {
                    (0..cols.parse().unwrap()).map(|i| {
                        html! {
                            <div
                                class="col-button"
                                disabled={cb_factory.is_game_finished()}
                                onclick = {
                                    cb_factory.get_callback_for(i)
                                }
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}

fn render_record_save(mutation_result: Option<MutationResult<UserMatchRecordMutation>>, is_guest: bool) -> Html {

    html! {
        <h6 class="subtitle is-6 has-text-centered">{
            match mutation_result {
                _ if is_guest => "Login to save match",
                None => "Saving...",
                Some(Ok(_)) => "Saved!",
                Some(Err(_)) => "Save Failed."
            }
        }</h6>
    }
}

