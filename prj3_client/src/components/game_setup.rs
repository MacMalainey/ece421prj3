use yew::prelude::*;

use crate::components::radio_group::RadioGroup;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub name: String, // name of game
    pub steps: Vec<String>, // steps to play game
    pub description: String, // description for game
    pub difficulties: Vec<String>, // types of difficulties for game
    pub board_sizes: Vec<String>, // types of board sizes
    pub disc_colors: Vec<String>, // types of disc colors that can be selected
    pub start_pressed: Callback<[String; 3]>, // start button callback
}

pub struct GameSetup {
    should_start: bool,
    selected_difficulty: String, // selected difficulty from radio group
    selected_board_size: String, // selected board size from radio group
    selected_disc_color: String, // selected disc color from radio group
}

pub enum Msg {
    StartPressed,
    UpdateDifficulty(String),
    UpdateBoardSize(String),
    UpdateDiscColor(String)
}
impl Component for GameSetup {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            should_start: false,
            selected_difficulty: ctx.props().difficulties.clone()[0].to_string(),
            selected_board_size: ctx.props().board_sizes.clone()[0].to_string(),
            selected_disc_color: ctx.props().disc_colors.clone()[0].to_string()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            // start button pressed
            Msg::StartPressed => {
                self.should_start = true;
                let start_pressed = ctx.props().start_pressed.clone();
                // emit start pressed to parent component
                start_pressed.emit([
                    self.selected_difficulty.clone(),
                        self.selected_board_size.clone(),
                        self.selected_disc_color.clone()]
                );
                true
            }

            // difficulty radio selected
            Msg::UpdateDifficulty(selection) => {
                self.selected_difficulty = selection;
                true
            }

            // board size radio selected
            Msg::UpdateBoardSize(selection) => {
                self.selected_board_size = selection;
                true
            }

            // disc color radio selected
            Msg::UpdateDiscColor(selection) => {
                self.selected_disc_color = selection;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name.clone();
        let steps = ctx.props().steps.clone();
        let description = ctx.props().description.clone();
        let difficulties = ctx.props().difficulties.clone();
        let board_sizes = ctx.props().board_sizes.clone();
        let disc_colors = ctx.props().disc_colors.clone();

        let setup_header = "Steps to play ".to_string() + &name + ": ";

        html! {
            <div class="container" style="max-width:850px">
                // name of game
                <h1 class="title has-text-centered mt-6">{name}</h1>
                // main content
                <div class="card mt-6">
                    <div class="columns">
                        // column for description
                        <div class="column background-1">
                            <div style="padding:25px">
                                <h3 class="subtitle color-4 has-text-centered">{"How to play"}
                                </h3>
                                <div class="is-size-6 mt-4 color-4">
                                    <div>{description}</div>
                                    <div class="mt-3">{setup_header}</div>
                                    <ul class="list">
                                        {
                                            // steps list for how to play
                                            steps.into_iter().map(|step| {
                                                html!{
                                                    <li class="list-item">{format!("- {}", step)}</li>
                                                }
                                            }).collect::<Html>()
                                        }
                                    </ul>
                                </div>
                            </div>
                        </div>
                        // column for options
                        <div class="column is-three-fifths background-5">
                            <div style="padding:25px">
                                <RadioGroup title={"Difficulties"} options={difficulties} name={"difficulty"} is_discs={false} update={ctx.link().callback(Msg::UpdateDifficulty).clone()}/>
                                <RadioGroup title={"Board sizes"} options={board_sizes} name={"board_size"} is_discs={false} update={ctx.link().callback(Msg::UpdateBoardSize).clone()} />
                                <RadioGroup title={"Disc colors"} options={disc_colors} name={"disc_color"} is_discs={true} update={ctx.link().callback(Msg::UpdateDiscColor).clone()} />
                                <div class="container" style={"width:90%; margin-left:0.75rem; margin-top: 190px;" }>
                                    <button class="button is-primary" onclick={ctx.link().callback(|_| Msg::StartPressed)} style={"width: 100%;"}>{"Start game"}</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
