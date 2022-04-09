use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub title: String,
    pub options: Vec<String>,
    pub name: String,
    pub is_discs: bool,
    pub update: Callback<String>,
}

pub struct RadioGroup {
    selected: String,
    index: usize
}

pub enum Msg {
    RadioSelected(usize),
}

impl Component for RadioGroup {
    type Properties = Props;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self{
            selected: ctx.props().options.clone()[0].clone(),
            index: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::RadioSelected(i) => {
                let opt = ctx.props().options.clone();
                let selected_str = opt[i].clone();
                self.selected = selected_str;
                self.index = i;
                let update = ctx.props().update.clone();
                update.emit(self.selected.clone());
                true
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = ctx.props().title.clone();
        let options = ctx.props().options.clone();
        let is_discs = ctx.props().is_discs.clone();

        html! {
            <>
                <div class="is-size-5 mt-4 mb-2 color-1">
                    {title}
                </div>
                <div class="control mb-6">
                    <label class="radio">
                        {
                            if is_discs {
                                options.iter().enumerate().map(|(i, option)| {
                                    html!{
                                        <span class="mx-2">
                                            <input
                                                class="color-1 mr-2"
                                                type="radio"
                                                name={ctx.props().name.clone()}
                                                onclick = {ctx.link().callback(move |_| Msg::RadioSelected(i))}
                                                checked = {self.index == i}
                                            />
                                            <text class="circle is-size-6" style={"color: ".to_string() + &option + &"; background-color:".to_string() + &option}>{"OO"}</text>
                                        </span>
                                    }
                                }).collect::<Html>()
                            } else {                                
                                options.iter().enumerate().map(|(i, option)| {
                                html!{
                                    <span class="mx-2 is-size-6">
                                        <input
                                            class="color-1 mr-2"
                                            type="radio"
                                            name={ctx.props().name.clone()}
                                            onclick = {ctx.link().callback(move |_| Msg::RadioSelected(i))}
                                            checked = {self.index == i}
                                            />
                                        {option}
                                    </span>
                                }}).collect::<Html>()
                            }
                        }
                    </label>
                </div>
            </>
        }
    }
}
