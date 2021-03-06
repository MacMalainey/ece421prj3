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
    /// Name of game
    pub name: String,
    /// Difficulty
    pub selected_difficulty: String,
    /// Disc Color
    pub selected_disc_color: String,
    /// Board size
    pub selected_board_size: String,
    /// Columns
    pub columns: String,
    /// Rows
    pub rows: String,
}

/// State for play screen
struct PlayScreenState {
    /// Game state that gets updated
    pub game_state: GameState,
    /// Board state that gets updated
    pub board_state: Vec<(i32, String)>,
    /// For TOOT and OTTO -> type of chip to place 
    pub is_t: bool, 
}

/// Factory object for creating callbacks when a column gets pressed
struct BoardUpdateCallbackFactory {
    /// The state handle for updating
    state: UseStateHandle<PlayScreenState>,
    /// Shared reference to the game object
    game: Rc<RefCell<Game>>,
    /// Mutation for writing the record when game ends
    record_mutation: UseMutationValueHandle<UserMatchRecordMutation>,
    /// Game type
    game_type: GameType,
    /// Cpu level
    cpu_level: CpuLevel,
    /// If the user is authenticated as guest
    is_guest: bool,
}

impl BoardUpdateCallbackFactory {
    /// Whether or not the game is finished
    fn is_game_finished(&self) -> bool {
        self.game.borrow().get_state() != GameState::Running
    }

    /// Get a callback for pressing a column
    fn get_callback_for<I>(&self, i: usize) -> Callback<I> {
        // Clone needed variables
        let game = self.game.clone();
        let state = self.state.clone();
        let record_mutation = self.record_mutation.clone();
        let game_id = self.game_type;
        let cpu_level = self.cpu_level;
        let is_guest = self.is_guest;
        // Make callback
        Callback::from(move |_| {
            let mut game_mut = game.borrow_mut();

            let letter = if state.is_t {
                Letter::T
            } else {
                Letter::O
            };

            if game_mut.player_turn(i, Some(letter)) {
                // Check for victory/tie
                let mut game_state = game_mut.check_state();

                if game_state == GameState::Running {
                    // Perform AI turn
                    game_mut.ai_turn();

                    // Check for loss/tie
                    game_state = game_mut.check_state();
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
                state.set(PlayScreenState { board_state, game_state, is_t: state.is_t }.into())
            }
        })
    }
}

/// Component for the play screen
#[function_component(PlayScreen)]
pub fn play_screen(props: &Props) -> Html {
    // Get props
    let name = props.name.clone();
    let selected_color = props.selected_disc_color.clone();
    let mode = props.selected_difficulty.clone();

    // Get AI config
    let (ai_config, cpu_level) = get_ai_config(&mode);

    // Spawn a reference to the game that stays throughout this component's lifetime
    let game = use_mut_ref(|| {
        Game::new(
            props.rows.clone().parse::<usize>().unwrap(),
            props.columns.clone().parse::<usize>().unwrap(),
            get_game_type(props.name.as_str()),
            ai_config
        )
    });

    // Get the state
    let state = {
        let game = game.clone();
        use_state(move || {
            PlayScreenState {
                game_state: GameState::Running,
                board_state: game.borrow().get_board_state(),
                is_t: true
            }
        })
    };

    // Get the user credentials
    let user = use_atom_value::<AuthCredentials>();

    // Get the handle for the match record save mutation
    let record_mutation = use_mutation_value::<UserMatchRecordMutation>();

    // Get game type
    let (p1, p2, game_type) = if name == "TOOT and OTTO" {
        ("You - TOOT", "Computer - OTTO", GameType::OttoToot)
    } else {
        ("You", "Computer", GameType::Connect4)
    };

    let is_guest = *user == AuthCredentials::Guest;

    // Create callback factory
    let cb_factory = BoardUpdateCallbackFactory {
        state: state.clone(),
        game: game.clone(),
        record_mutation: record_mutation.clone(),
        game_type,
        cpu_level,
        is_guest
    };

    // Callback for selecting t
    let on_t_selected = {
        let state = state.clone();
        let game = game.clone();
        Callback::from(move |_| {
            state.set(PlayScreenState {
                board_state: game.borrow().get_board_state(),
                game_state: GameState::Running,
                is_t: true
            }.into());
        })
    };

    // Callback for selecting o
    let on_o_selected = {
        let state = state.clone();
        let game = game.clone();
        Callback::from(move |_| {
            state.set(PlayScreenState {
                board_state: game.borrow().get_board_state(),
                game_state: GameState::Running,
                is_t: false
            }.into());
        })
    };

    // Get the game type
    let game_type = get_game_type(name.as_str());
    let is_toot_and_otto = game_type == GameType::OttoToot;

    html! {
        <div class="container" style="max-width:650px">
            <h1 class="title has-text-centered mt-6">{name}</h1>
            <div class="mt-6">
                //p1 name and color
                <div class="in-game-player-info">
                    <div style={"height: 15px; width: 15px; border-radius: 50%; background-color:".to_string() + &selected_color.to_string()}/>
                    <div style={""}>{p1}</div>
                </div>
                //p2 name and color
                <div class="in-game-player-info">
                    <div style={"height: 15px; width: 15px; border-radius: 50%; background-color:".to_string() + get_opponent_color(selected_color.to_string(), is_toot_and_otto)}/>
                    <div style={""}>{p2}</div>
                </div>
                {   //if toot and otto, show T and O selection radios
                    if props.name.clone() != "Connect 4".to_string() {
                        html! {
                            <div class="in-game-player-info ml-5" style={"float:right"}>
                                <div>{"Select letter: "}</div>
                                    //T radio
                                    <span class="mx-2 is-size-6">
                                        <input
                                            class="color-1 mr-2"
                                            type="radio"
                                            onclick = {on_t_selected}
                                            checked = {state.clone().is_t}
                                            />
                                        {"T"}
                                    </span>
                                    //O radio
                                    <span class="mx-2 is-size-6">
                                        <input
                                            class="color-1 mr-2"
                                            type="radio"
                                            onclick = {on_o_selected}
                                            checked = {!state.clone().is_t}
                                            />
                                        {"O"}
                                    </span>
                            </div>
                        }
                    } else {html!{}}
                }
            </div>
            <div class="card mt-2">
                {
                    // Render the grid
                    render_grid(
                        props.selected_board_size.clone(),
                        state.board_state.clone(),
                        props.selected_disc_color.clone(),
                        props.name.clone(),
                    )
                }
                {
                    if state.game_state == GameState::Running {
                        // Render the buttons on top of the grid
                        render_col_buttons(
                            cb_factory,
                            props.selected_board_size.clone(),
                        )
                    } else {
                        // Render the result screen
                        let result = get_result_text(state.game_state).to_string();

                        // Restart game state callback
                        let on_restart_clicked = {
                            let props = props.clone();
                            Callback::from(move |_| {
                                *(game.borrow_mut()) = Game::new(
                                    props.rows.clone().parse::<usize>().unwrap(),
                                    props.columns.clone().parse::<usize>().unwrap(),
                                    get_game_type(props.name.as_str()),
                                    ai_config
                                );
                                state.set(PlayScreenState {
                                    board_state: game.borrow().get_board_state(),
                                    game_state: GameState::Running,
                                    is_t: true
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
                <div style={"float:right"}>{format!("{} mode", mode)}</div>
            </div>
        </div>
    }
}

/// Parses the game type
fn get_game_type(diff: &str) -> GameType {
    match diff {
        "TOOT and OTTO" => {
            GameType::OttoToot
        },
        _ => {
            GameType::Connect4
        }
    }
}

/// Parses the AI config
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

/// Returns appropriate result text
fn get_result_text(state: GameState) -> &'static str {
    if state == GameState::Tie {
        "You tied"
    } else if state == GameState::Win(PLAYER_ID) {
        "You win!"
    } else {
        "You lost :("
    }
}

/// Returns the opponent's color as a hex string
fn get_opponent_color(selected_disc_color: String, is_same: bool) -> &'static str {
    if is_same {
        if selected_disc_color == "#FF8E8E" {
            "#FF8E8E"
        } else if selected_disc_color == "#FFE68E" {
            "#FFE68E"
        } else {
            "#FF8E8E"
        }
    } else if selected_disc_color == "#FF8E8E" {
        "#FFE68E"
    } else {
        "#FF8E8E"
    }
}

/// Renders the board game grid
fn render_grid(selected_board_size: String, board_state: Vec<(i32, String)>, selected_disc_color: String, name: String) -> Html {
    //get game type toot and otto or connect 4
    let game_type = get_game_type(name.as_str());
    let is_toot_and_otto = game_type == GameType::OttoToot;

    let split: Vec<&str> = selected_board_size.split("x").collect();
    let cols = split[0];
    // let rows = split[1];
    html! {
        <>
            <div class={"background-3 grid-container grid_cols_".to_string() + &cols.to_string()}>
                {
                    //render p1 circle pieces and letter
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
                        //render p2 circle pieces and letter
                        else if piece == 2 {
                            html! {
                                <div class="grid-item">
                                    <div class="circle" style={"background-color:".to_string() + get_opponent_color(selected_disc_color.to_string(), is_toot_and_otto)}>
                                        <div>{letter}</div>
                                    </div>
                                </div>
                            }
                        //render white circle pieces
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

/// Renders the column buttons
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

/// Renders the record save text
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

