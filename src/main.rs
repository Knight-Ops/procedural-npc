use bnf::Grammar;
use std::{fs, io};

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

fn main() {
    let input = read_bnfs().expect("Error while reading BNF files!");

    let grammar: Grammar = input.parse().unwrap();

    for _ in 0..10 {
        let npc = grammar.generate().unwrap();
        println!("{}", npc);

        println!("------------------")
    }
}
