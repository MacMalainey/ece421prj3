use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

use crate::game;
use crate::game::*;

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

#[function_component(PlayScreen)]
pub fn play_screen(props: &Props) -> Html {
    let name = props.name.clone();
    let selected_color = props.selected_disc_color.clone();
    let mode = props.selected_difficulty.clone();

    let game = use_mut_ref(|| {
        Game::new(
            props.rows.clone().parse::<usize>().unwrap(),
            props.columns.clone().parse::<usize>().unwrap(),
            get_ai_config(&mode)
        )
    });

    let state = use_state(|| {
        let board_state = game.borrow().get_board_state();

        PlayScreenState {
            game_state: GameState::Running,
            board_state,
        }
    });

    let mut p1 = "You";
    let mut p2 = "Computer";
    if name == "TOOT and OTTO" {
        p1 = "You - TOOT";
        p2 = "Computer - OTTO";
    }

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
                            state.clone(),
                            game,
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
                                    get_ai_config(&mode)
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
                                    <button class="button is-primary" onclick={on_restart_clicked} style={"width: 100%;"}>{"Play again"}</button>
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

fn get_ai_config(diff: &str) -> AIConfiguration {
    match diff {
        "Easy" => {
            game::AI_EASY
        },
        "Medium" => {
            game::AI_MEDIUM
        },
        "Hard" => {
            game::AI_HARD
        },
        _ => {
            game::AI_EASY
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

fn render_col_buttons(state: UseStateHandle<PlayScreenState>, game: Rc<RefCell<Game>>, selected_board_size: String) -> Html {
    let split: Vec<&str> = selected_board_size.split("x").collect();
    let cols = split[0];
    html! {
        <>
            <div class={"col-button-container grid-container grid_cols_".to_string() + &cols.to_string()}>
                {
                    (0..cols.parse().unwrap()).map(|i| {
                        let state = state.clone();
                        html! {
                            <div
                                class="col-button"
                                onclick = {
                                    let game = game.clone();
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

                                            state.set(PlayScreenState {
                                                board_state: game_mut.get_board_state(),
                                                game_state
                                            }.into())
                                        }
                                    })
                                }
                            />
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}

