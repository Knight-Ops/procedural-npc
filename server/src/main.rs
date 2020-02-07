#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use maud::{html, Markup};
use rocket::response::status;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;

use grammars::NPC;
use grammars::Gender;

mod types;

#[get("/static/<path..>")]
fn static_file(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/ui.wasm")]
fn hacky_hack() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/ui.wasm")).ok()
}

#[get("/npc")]
fn random_npc() -> String {
    let npc = NPC::default();

    serde_json::to_string(&npc).expect("Error while serializing random_npc")
}

#[get("/")]
fn index() -> Markup {
    html! {
        link rel="stylesheet" href="static/style.css" {}
        body {}
        script src=("static/ui.js") {}
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, random_npc, static_file, hacky_hack]).launch();
}
