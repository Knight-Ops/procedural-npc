use bnf::Grammar;
use std::{fs, io};

fn read_bnfs() -> io::Result<String> {
    let mut temp_str = String::new();

    let mut files = fs::read_dir("src/grammars")?.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;

    files.sort();

    for file in files {
        temp_str += &fs::read_to_string(file)?;
    }

    Ok(temp_str)
}

fn main() {
    let input = read_bnfs().expect("Error while reading BNF files!");

    let grammar: Grammar = input.parse().unwrap();

    for _ in 0..10 {
        let npc = grammar.generate().unwrap();
        println!("{}", npc);

        println!("------------------")
    }
}
