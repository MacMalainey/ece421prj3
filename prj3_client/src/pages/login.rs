use yew::html::Scope;
use yew::prelude::*;

pub struct Login {
    is_on_login: bool,
}

pub enum Msg {
    LoginPressed,
    SignupPressed,
}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {is_on_login: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::LoginPressed => {
                self.is_on_login = true;
                true
            }
            Msg::SignupPressed => {
                self.is_on_login = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            {self.view_login(ctx.link()) }
            </>
        }
    }
}

impl Login {
    fn view_login(&self, link: &Scope<Self>) -> Html {
        let Self {is_on_login, .. } = *self;
        let login_class;
        let signup_class;
        let button_text;
        if is_on_login {
            login_class = "is-active";
            signup_class = "";
            button_text = "Login"
        } else {
            login_class = "";
            signup_class = "is-active";
            button_text = "Create Account"
        }

        html! {
            <div class="container is-max-desktop center-form">
                    <h1 class="title has-text-centered mt-6">{"Boardgames"}</h1>
                    <div class="tabs mt-6">
                      <ul>
                        <li class={login_class} onclick={link.callback(|_| Msg::LoginPressed)}>
                          <a>{"Login"}</a>
                        </li>
                        <li class={signup_class} onclick={link.callback(|_| Msg::SignupPressed)}>
                          <a>{"Sign up"}</a>
                        </li>
                      </ul>
                    </div>
                    <form>
                      <div class="field mt-4">
                        <label class="label">{"Username"}</label>
                        <div class="control">
                          <input class="input" type="username" placeholder="e.g. lora"/>
                        </div>
                      </div>
                      <div class="field mt-4">
                        <label class="label">{"Password"}</label>
                        <div class="control">
                          <input class="input" type="password" placeholder="********"/>
                        </div>
                      </div>

                      <button class="button is-primary mt-4">{button_text}</button>
                    </form>
            </div>
        }
    }
}