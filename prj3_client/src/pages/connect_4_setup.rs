use yew::prelude::*;

use crate::components::{
    game_setup::GameSetup,
    play_screen::PlayScreen,
};

pub struct Connect4Setup {
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
    board_state: Vec<(i32, String)>,
}

pub enum Msg {
    StartPressed([String; 3]),
}

impl Component for Connect4Setup {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            name: "Connect 4".to_string(),
            description: "Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs.
            ".to_string(),
            steps: vec!["A new game describes discs of which color belongs to which player".to_string(),
            "Click on the desired column on the game board to place your disc".to_string(),
            "Try to connect 4 of your colored discs either horizontally or vertically or diagonally".to_string()],
            difficulties: vec!["Easy".to_string(), "Medium".to_string(), "Hard".to_string()],
            board_sizes: vec!["5x4".to_string(), "7x6".to_string()],
            disc_colors: vec!["#FF8E8E".to_string(), "#FFE68E".to_string(), "black".to_string()],
            selected_difficulty: "Easy".to_string(),
            selected_disc_color: "#FF8E8E".to_string(),
            selected_board_size: "5x4".to_string(),
            should_start: false,
            board_state: vec![(0, "".to_string()), (0, "".to_string()),  (0, "".to_string()), (0, "".to_string()), (0, "".to_string()),
                              (0, "".to_string()), (0, "".to_string()), (0, "".to_string()), (0, "".to_string()), (0, "".to_string()),
                              (0, "".to_string()), (0, "".to_string()), (0, "".to_string()), (0, "".to_string()), (0, "".to_string()),
                              (0, "".to_string()), (0, "".to_string()), (0, "".to_string()), (1, "".to_string()), (2, "".to_string())]
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
                <PlayScreen
                    name={self.name.clone()}
                    selected_difficulty = {self.selected_difficulty.clone()}
                    selected_board_size = {self.selected_board_size.clone()}
                    selected_disc_color = {self.selected_disc_color.clone()}
                    board_state = {self.board_state.clone()}
                />
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
