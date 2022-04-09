use yew::prelude::*;
use crate::Connect4Setup;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub selected_difficulty: String,
    pub selected_disc_color: String,
    pub selected_board_size: String,
    pub board_state: Vec<i32>,
}

pub struct Connect4Play {
}

pub enum Msg {
}

impl Component for Connect4Play {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name.clone();
        html! {
            <div class="container" style="max-width:650px">
                <h1 class="title has-text-centered mt-6">{name}</h1>
                <div class="card mt-6">
                    {self.render_grid(
                        ctx.props().selected_board_size.clone(),
                        ctx.props().board_state.clone(),
                        ctx.props().selected_disc_color.clone())}
                </div>
            </div>
        }
    }
}

impl Connect4Play {
    fn get_opponent_color(&self, selected_disc_color: String) -> &'static str {
        if selected_disc_color == "#FF8E8E" {
            "#FFE68E"
        } else {
            "#FF8E8E"
        }
    }
    fn render_grid(&self, selected_board_size: String, board_state: Vec<i32>, selected_disc_color: String) -> Html {
        let mut split:  Vec<&str> = selected_board_size.split("x").collect();
        let cols = split[0];
        let rows = split[1];
        html! {
            <>
                <div class={"background-3 grid-container grid_cols_".to_string() + &cols.to_string()}>
                    {
                         board_state.into_iter().map(|piece| {
                            if piece == 1 {
                                html!{
                                    <div class="grid-item">
                                        <div class="circle" style={"background-color:".to_string() + &selected_disc_color.to_string()}/>
                                    </div>
                                }
                            }
                            else if piece == 2 {
                                html! {
                                    <div class="grid-item">
                                        <div class="circle" style={"background-color:".to_string() + self.get_opponent_color(selected_disc_color.to_string())}/>
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
}
