use yew::html::Scope;
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

pub struct PlayScreen {
    pub game: Game,
    pub game_state: GameState,
    pub board_state: Vec<(i32, String)>,
    pub ai: AIConfiguration,
}

pub enum Msg {
    ColumnSelected(usize),
    RestartGame
}

impl Component for PlayScreen {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ai = match ctx.props().selected_difficulty.as_str() {
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
        };

        let game = Game::new(
            ctx.props().rows.clone().parse::<usize>().unwrap(),
            ctx.props().columns.clone().parse::<usize>().unwrap(),
            ai,
        );

        let board_state = game.get_board_state();

        Self {
            game,
            game_state: GameState::Running,
            board_state,
            ai
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::ColumnSelected(column) => {
                // log::info!("Player turn");
                if self.game.player_turn(column) {
                    // Update render
                    self.board_state = self.game.get_board_state();

                    // Check for victory/tie
                    self.game_state = self.game.check_state(game::PLAYER_ID);

                    if self.game_state != GameState::Running {
                        return true;
                    }

                    // Perform AI turn
                    self.game.ai_turn();

                    // Update render
                    self.board_state = self.game.get_board_state();

                    // Check for victory/tie
                    self.game_state = self.game.check_state(game::AI_ID);
                }

                true
            }
            Msg::RestartGame => {
                self.game = Game::new(
                    ctx.props().rows.clone().parse::<usize>().unwrap(),
                    ctx.props().columns.clone().parse::<usize>().unwrap(),
                    self.ai,
                );

                self.board_state = self.game.get_board_state();
                self.game_state = GameState::Running;

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name.clone();
        let selected_color = ctx.props().selected_disc_color.clone();
        let mode = ctx.props().selected_difficulty.clone();

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
                        <div style={"height: 15px; width: 15px; border-radius: 50%; background-color:".to_string() + self.get_opponent_color(selected_color.to_string())}/>
                        <div style={""}>{p2}</div>
                    </div>
                    <div style={"float:right"}>{format!("{} mode", mode)}</div>
                </div>
                <div class="card mt-2">
                    {
                        self.render_grid(
                            ctx.props().selected_board_size.clone(),
                            self.board_state.clone(),
                            ctx.props().selected_disc_color.clone()
                        )
                    } {
                        if self.game_state == GameState::Running {
                            self.render_col_buttons(
                                ctx.link(),
                                ctx.props().selected_board_size.clone(),
                            )
                        } else {
                            let result = self.get_result_text(self.game_state).to_string();
                            html! {
                                <div class="card results-card">
                                  <div class="card-content">
                                    <div class="content">
                                        <h1 class="title has-text-centered">{result}</h1>
                                        <button class="button is-primary" onclick={ctx.link().callback(|_| Msg::RestartGame)} style={"width: 100%;"}>{"Play again"}</button>
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
}

impl PlayScreen {
    fn get_result_text(&self, state: GameState) -> &'static str {
        if state == GameState::Tie {
            "You tied"
        } else if state == GameState::Win(PLAYER_ID) {
            "You win!"
        } else {
            "You lost :("
        }
    }

    fn get_opponent_color(&self, selected_disc_color: String) -> &'static str {
        if selected_disc_color == "#FF8E8E" {
            "#FFE68E"
        } else {
            "#FF8E8E"
        }
    }
    fn render_grid(&self, selected_board_size: String, board_state: Vec<(i32, String)>, selected_disc_color: String) -> Html {
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
                                        <div class="circle" style={"background-color:".to_string() + self.get_opponent_color(selected_disc_color.to_string())}>
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

    fn render_col_buttons(&self, link: &Scope<Self>, selected_board_size: String) -> Html {
        let split: Vec<&str> = selected_board_size.split("x").collect();
        let cols = split[0];
        // let rows = split[1];
        let iterator: Vec<i32> = (0..cols.parse().unwrap()).collect();
        html! {
            <>
                <div class={"col-button-container grid-container grid_cols_".to_string() + &cols.to_string()}>
                    {
                        iterator.iter().enumerate().map(|(i, _)| {
                            html! {
                                <div
                                    class="col-button"
                                    onclick = {link.callback(move |_| Msg::ColumnSelected(i))}
                                />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </>
        }
    }
}
