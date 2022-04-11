use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub title: String, //name for group
    pub options: Vec<String>, //vector of possible choices
    pub name: String, //name for radio
    pub is_discs: bool, //is disc color group
    pub update: Callback<String>, //callback for if radio is selected
}

pub struct RadioGroup {
    selected: String, //selected radio text
    index: usize //selected radio index
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
            //when a radio is selected in the group
            Msg::RadioSelected(i) => {
                let opt = ctx.props().options.clone();
                let selected_str = opt[i].clone();

                //set new selected string and index
                self.selected = selected_str;
                self.index = i;

                //send selected string to parent component
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
                    //radio group
                    <label class="radio">
                        {   //render disc group
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
                                //render radio options
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
