use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::console::ConsoleService;
use yew::format::{Json, Nothing};
use serde_derive::{Deserialize, Serialize};
use failure::Error;

use grammars::NPC;

struct Model {
    fetch_service: FetchService,
    console_service: ConsoleService,
    link: ComponentLink<Self>,
    fetching: bool,
    data: Option<NPC>,
    ft: Option<FetchTask>,
}

enum Msg {
    NewNPC,
    FetchReady(Result<NPC, Error>),
    Ignore
}

impl Model {
    fn view_data(&self) -> Html {
        if self.data.is_some() {
            let value = format!("{:?}", self.data.as_ref().unwrap());
            
            html! {
                <p>{ value }</p>
            }

        } else {
            html! {
                <p>{ "Data hasn't been fetched yet!" }</p>
            }
        }
    }

    fn fetch_json(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<NPC, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                // self.console_service.log(&format!("META: {:?}, {:?}", meta, data));
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore
                }
            }
        );

        let request = Request::get("/npc").body(Nothing).unwrap();
        self.fetch_service.fetch(request, callback)
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { 
            fetch_service: FetchService::new(),
            console_service: ConsoleService::new(),
            link: link,
            fetching: false,
            ft: None,
            data: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewNPC => {
                self.console_service.log("NewNPC");
                self.fetching = true;
                let task = self.fetch_json();

                self.ft = Some(task);
                false
            }
            Msg::FetchReady(response) => {
                self.console_service.log("FetchReady");
                self.fetching = false;
                self.data = response.map(|data| data).ok();
                true
            }
            Msg::Ignore => {
                self.console_service.log("Ignore");
                false
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::NewNPC)>{ "New NPC" }</button>
                    { self.view_data() }
                </nav>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}