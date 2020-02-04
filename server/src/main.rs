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

use bnf::Grammar;
use std::{fs, io};
use serde_json::Value;

fn read_bnfs() -> io::Result<String> {
    let mut temp_str = String::new();

    let mut files = fs::read_dir("src/grammars")?.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;

    files.sort();

    for file in files {
        temp_str += &fs::read_to_string(file)?;
    }

    // Filter out comments so we don't panic
    let mut bnf_lines : Vec<&str> = temp_str.lines().collect();
    
    for (idx, line) in bnf_lines.clone().iter().enumerate() {
        if line.starts_with('#') {
            bnf_lines.remove(idx);
        }
    }

    let mut uncommented_string : String = String::new();

    bnf_lines.iter().for_each(|entry| uncommented_string += *entry);

    Ok(uncommented_string)
}

// fn old_main() {
//     let input = read_bnfs().expect("Error while reading BNF files!");

//     let grammar: Grammar = input.parse().unwrap();

//     for _ in 0..100 {
//         let npc = grammar.generate().unwrap();

//         let v: Value = serde_json::from_str(&npc).unwrap();

//         if format!("{}", v["Race"]).contains("Dwarf") {
//             println!("{}", v["Name"])
//         }

//         // println!("{}", npc);

//         // println!("------------------")
        
//     }
// }
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
    let input = read_bnfs().expect("Error while reading BNF files!");

    let grammar: Grammar = input.parse().unwrap();

    let npc = grammar.generate().unwrap();

    npc.to_owned()
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
    random_npc();
    rocket::ignite().mount("/", routes![index, random_npc, static_file, hacky_hack]).launch();
}
