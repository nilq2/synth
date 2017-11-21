mod tokenizer;
use tokenizer::tokenizer::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let mut s = Source::new(&args[1], Some("//!"));

        tokenize(&mut s);

        for token in s.tokens.unwrap() {
            println!("{:#?}: {:?} ", token.token_type, token.lexeme)
        }

    } else {
        println!("source file not supplied: synth <file>")
    }
}
