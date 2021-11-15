mod command;

use web_sys::*;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew::html::Scope;
use gloo_console as console;


struct Model {
    link: ComponentLink<Self>,
    str: String,
    past: Vec<String>
}

enum Msg {
    Str(String),
    Enter,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            str: "".to_string(),
            past: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Enter => {
                console::log!(self.str.clone());
                command::command_check(self.str.clone());
                self.past.push("shallowfeather@sf: ".to_string() + &*self.str.to_string());
                self.past.push('\n'.to_string());
                true
            }
            Msg::Str(str) => {
                self.str = str.clone();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <li style="white-space: pre-line; list-style-type:none;"> { for self.past.iter() } </li>
                <div>
                    <span> { "shallowfeather@sf: " } </span>
                    <input
                        oninput=self.link.callback(|e: InputData|
                            Msg::Str(e.value)
                        )
                        onkeypress=self.link.batch_callback(|e: KeyboardEvent|{
                            if e.key() == "Enter" {
                                Some(Msg::Enter)
                            }
                            else {
                                None
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}



fn main() {
    yew::start_app::<Model>();
}