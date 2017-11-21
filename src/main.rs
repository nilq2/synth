use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod tokenizer;
use tokenizer::tokenizer::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let f: File = match File::open(&args[1]) {
            Ok(v) => v,
            Err(_) => panic!("no such file: {}", &args[1]),
        };

        let file  = BufReader::new(&f);
        let lines = file.lines().map(|x| x.unwrap()).collect();

        let mut s = Source::new(&args[1], Some("//!"), &lines);

        tokenize(&mut s);

        for token in s.tokens.unwrap() {
            println!("{:#?}: {:?} ", token.token_type, token.lexeme)
        }

    } else {
        println!("source file not supplied: synth <file>")
    }
}
