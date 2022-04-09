use yew::prelude::*;

pub struct Leaderboard {
    isOnConnect4: bool,
}

pub enum Msg {
    SwitchToConnect,
    SwitchToToot
}

impl Component for Leaderboard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            isOnConnect4: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::SwitchToConnect => {
                self.isOnConnect4 = true;
                true
            }
            Msg::SwitchToToot => {
                self.isOnConnect4 = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut connect_class = "is-active";
        let mut toot_class = "";

        if !self.isOnConnect4 {
            connect_class = "";
            toot_class = "is-active";
        }

        html! {
            <div class="tabs is-centered is-boxed">
              <ul>
                <li class={connect_class} onclick={ctx.link().callback(|_| Msg::SwitchToConnect)}>
                  <a>
                    <span>{"Connect 4"}</span>
                  </a>
                </li>
                <li class={toot_class} onclick={ctx.link().callback(|_| Msg::SwitchToToot)}>
                  <a>
                    <span>{"TOOT OTTO"}</span>
                  </a>
                </li>
              </ul>
            </div>
        }
    }
}

