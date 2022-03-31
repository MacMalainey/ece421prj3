use yew::html::Scope;
use yew::prelude::*;

use crate::components::{
    game_setup::GameSetup,
};

pub struct TOOTSetup {
    name: String,
    description: String,
    difficulties: Vec<String>,
    board_sizes: Vec<String>,
    disc_colors: Vec<String>,
}

pub enum Msg {}

impl Component for TOOTSetup {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            name: "TOOT and OTTO".to_string(),
            description: "
            TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!
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
