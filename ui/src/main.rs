use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>{ "Click me!" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}