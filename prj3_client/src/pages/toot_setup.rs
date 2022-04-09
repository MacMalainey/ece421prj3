use yew::prelude::*;

use crate::components::game_setup::GameSetup;

pub struct TootSetup {
    name: String,
    steps: Vec<String>,
    description: String,
    difficulties: Vec<String>,
    board_sizes: Vec<String>,
    disc_colors: Vec<String>,
    selected_difficulty: String,
    selected_disc_color: String,
    selected_board_size: String,
    should_start: bool,
}

pub enum Msg {
    StartPressed([String; 3]),
}

impl Component for TootSetup {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "TOOT and OTTO".to_string(),
            description: "TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!".to_string(),
            steps: vec!["A new game describes which player is TOOT and which is OTTO".to_string(),
                        "Select the disc type T or O that you want to place".to_string(),
                        "Click on the desired column on the game board to place your disc".to_string(),
                        "Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally".to_string()],
            difficulties: vec!["Easy".to_string(), "Medium".to_string(), "Hard".to_string()],
            board_sizes: vec!["5x4".to_string(), "7x6".to_string()],
            disc_colors: vec!["#FF8E8E".to_string(), "#FFE68E".to_string(), "black".to_string()],
            selected_difficulty: "Easy".to_string(),
            selected_disc_color: "5x4".to_string(),
            selected_board_size: "#FF8E8E".to_string(),
            should_start: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::StartPressed(selections) => {
                self.selected_difficulty = selections[0].clone();
                self.selected_board_size = selections[1].clone();
                self.selected_disc_color = selections[2].clone();
                self.should_start = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {{if self.should_start {
            html! {
                <div/>
            }
        } else {
            html! {
                <GameSetup
                    name = {self.name.clone()}
                    steps= {self.steps.clone()}
                    description = {self.description.clone()}
                    difficulties = {self.difficulties.clone()}
                    board_sizes = {self.board_sizes.clone()}
                    disc_colors = {self.disc_colors.clone()}
                    start_pressed = {ctx.link().callback(Msg::StartPressed).clone()}
                />
            }
        }}}
    }
}
