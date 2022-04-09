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
    pub board_state: Vec<(i32, String)>,
}

pub enum Msg {}

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
            ai
        );

        let board_state = game.get_board_state();

        Self {
            game,
            board_state,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
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
                        self.render_col_buttons(
                            ctx.props().selected_board_size.clone(),
                        )
                    }
                </div>
            </div>
        }
    }
}

impl PlayScreen {
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

    fn render_col_buttons(&self, selected_board_size: String) -> Html {
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
                                <div class="col-button"/>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </>
        }
    }
}
