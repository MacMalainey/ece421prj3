use yew::html::Scope;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub options: Vec<String>,
    pub name: String,
    pub isDiscs: bool,
}

pub struct RadioGroup {

}

pub enum Msg {}

impl Component for RadioGroup {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = ctx.props().title.clone();
        let options = ctx.props().options.clone();
        let isDiscs = ctx.props().isDiscs.clone();
        html! {
            <>
                <div class="is-size-6 mt-4 color-1">
                    {title}
                </div>
                <div class="control mb-4">
                    <label class="radio">
                        {
                            if isDiscs {
                                options.into_iter().map(|option| {
                                    html!{
                                        <span class="mx-2">
                                            <input class="color-1 mr-2" type="radio" name={ctx.props().name.clone()}/>
                                            <text class="circle" style={"color: ".to_string() + &option + &"; background-color:".to_string() + &option}>{"OO"}</text>
                                        </span>
                                    }
                                }).collect::<Html>()
                            } else {                                
                                options.into_iter().map(|option| {
                                    html!{
                                        <span class="mx-2">
                                            <input class="color-1 mr-2" type="radio" name={ctx.props().name.clone()}/>
                                            {option}
                                        </span>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </label>
                </div>
            </>
        }
    }
}
