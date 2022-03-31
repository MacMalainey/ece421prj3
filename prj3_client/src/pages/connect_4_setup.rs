use yew::html::Scope;
use yew::prelude::*;

use crate::components::{
    game_setup::GameSetup,
};

pub struct Connect4Setup {
    name: String,
    description: String,
    difficulties: Vec<String>,
    board_sizes: Vec<String>,
    disc_colors: Vec<String>,
}

pub enum Msg {}

impl Component for Connect4Setup {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            name: "Connect 4".to_string(),
            description: "Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs.
            ".to_string(),
            difficulties: vec!["Easy".to_string(), "Medium".to_string(), "Hard".to_string()],
            board_sizes: vec!["5x4".to_string(), "7x6".to_string()],
            disc_colors: vec!["#FF8E8E".to_string(), "#FFE68E".to_string(), "#9284CC".to_string(), "#000000".to_string()]
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <GameSetup
                name = {self.name.clone()}
                description = {self.description.clone()}
                difficulties = {self.difficulties.clone()}
                board_sizes = {self.board_sizes.clone()}
                disc_colors = {self.disc_colors.clone()}
            />
        }
    }
}
