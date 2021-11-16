mod command;
mod control;

use web_sys::*;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
use yew::html::Scope;
use gloo_console as console;
use std::borrow::{BorrowMut, Borrow};
use std::ops::DerefMut;
use std::collections::HashMap;


pub struct Model {
    link: ComponentLink<Self>,
    str: String,
    pastmsg: Vec<String>,
    path: String,
    next: HashMap<String, Vec<String>>,
    is_root: bool,
}

pub enum Msg {
    Str(String),
    Enter,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut map = HashMap::new();
        Self {
            link,
            str: "".to_string(),
            pastmsg: Vec::new(),
            path: "~".to_string(),
            next: map,
            is_root: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Enter => {
                console::log!(self.str.clone());
                self.pastmsg.push("shallowfeather@sf: ".to_string() + &*self.str.to_string());
                self.pastmsg.push('\n'.to_string());
                let Split = self.str.split(" ").clone();
                let Input = Split.collect::<Vec<&str>>();
                let now = self.path.split("/");
                let now = now.collect::<Vec<&str>>();
                if Input.len() == 1 {
                    if Input[0].to_string() == "help" {
                        self.pastmsg.push("pwd".to_string() + &*'\n'.to_string());
                    }
                    else if Input[0].to_string() == "pwd" {
                        self.pastmsg.push(self.path.clone());
                    }
                    else if Input[0].to_string() == "ls" {
                        console::log!(now[now.len() - 1]);
                        let num = now[now.len() - 1].to_string();
                        match &self.next.get(&num) {
                            Some(val) => {
                                for i in &self.next[now[now.len() - 1]] {
                                    self.pastmsg.push(i.to_string());
                                    self.pastmsg.push('\n'.to_string());
                                }
                            }
                            None => {
                                self.pastmsg.push("Empty".to_string());
                            }
                        }
                    }
                    else {
                        self.pastmsg.push(self.str.clone() + ": command not found" + &*'\n'.to_string());
                    }
                } else {
                    if Input[0].to_string() == "mkdir" {
                        console::log!(now.len() - 1);
                        let mut check = false;
                        let num = now[now.len() - 1].to_string();
                        match &self.next.get(&num) {
                            Some(val) => {
                            }
                            None => {
                                self.next.insert(now[now.len() - 1].to_string(), Vec::new());
                            }
                        }
                        for i in &self.next[now[now.len() - 1]] {
                            if i == Input[1] {
                                self.pastmsg.push("Error".to_string());
                                check = true;
                                break;
                            }
                        }
                        if check == false {
                            self.next.entry(now[now.len() - 1].to_string())
                                .or_insert_with(|| Vec::new())
                                .push(Input[1].to_string());
                        }
                    }
                    else if Input[0].to_string() == "cd" {
                        let mut check = false;
                        for i in &self.next[now[now.len() - 1]] {
                            if i == Input[1] {
                                check = true;
                                break;
                            }
                        }
                        if(check == false) {
                            self.pastmsg.push("Error".to_string());
                        }
                        else {
                            self.path = self.path.clone() + &*"/".to_string() + Input[1];
                        }
                    }
                    else { self.pastmsg.push(self.str.clone() + ": command not found" + &*'\n'.to_string()); }
                }
                self.pastmsg.push('\n'.to_string());
                self.str = "".to_string();
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
                <li style="white-space: pre-line; list-style-type:none;" id="terminal__prompt--user"> { for self.pastmsg.iter() } </li>
                <div>
                    <span id="terminal__prompt--user"> { "shallowfeather@sf $: " } </span>
                    <span id="terminal__prompt--user"> { self.str.clone() } </span>
                    <input style="opacity: 0;"
                        value={self.str.clone()}
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