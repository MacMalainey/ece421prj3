use yew::prelude::*;

use crate::components::radio_group::RadioGroup;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub name: String,
    pub description: String,
    pub difficulties: Vec<String>,
    pub board_sizes: Vec<String>,
    pub disc_colors: Vec<String>,
}

pub struct GameSetup {
}

impl Component for GameSetup {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = ctx.props().name.clone();
        let description = ctx.props().description.clone();
        let difficulties = ctx.props().difficulties.clone();
        let board_sizes = ctx.props().board_sizes.clone();
        let disc_colors = ctx.props().disc_colors.clone();
        html! {
            <div class="container" style="max-width:850px">
                <h1 class="title has-text-centered mt-6">{name}</h1>
                <div class="card mt-6">
                    <div class="columns">
                        <div class="column background-1">
                            <div style="padding:25px">
                                <h3 class="subtitle color-4 has-text-centered">{"How to play"}
                                </h3>
                                <div class="is-size-6 mt-4 color-4">
                                    {description}
                                </div>
                            </div>
                        </div>
                        <div class="column is-three-fifths background-5">
                            <div style="padding:25px">
                                <RadioGroup title={"Difficulties"} options={difficulties} name={"difficulty"} is_discs={false}/>
                                <RadioGroup title={"Board sizes"} options={board_sizes} name={"board_size"} is_discs={false}/>
                                <RadioGroup title={"Disc colors"} options={disc_colors} name={"disc_color"} is_discs={true}/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
