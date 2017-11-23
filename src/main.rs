use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod tokenizer;
mod parser;

use tokenizer::tokenizer::*;
use parser::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let template_f: File = match File::open(&args[1]) {
            Ok(v) => v,
            Err(_) => panic!("no such file: {}", &args[1]),
        };

        let template_raw  = BufReader::new(&template_f);
        let template_lines = template_raw.lines().map(|x| x.unwrap()).collect();

        let mut template = Source::new(&args[1], Some("//!"), &template_lines);


        let unit_f: File = match File::open(&args[2]) {
            Ok(v) => v,
            Err(_) => panic!("no such file: {}", &args[2]),
        };

        let unit_raw  = BufReader::new(&unit_f);
        let unit_lines = unit_raw.lines().map(|x| x.unwrap()).collect();

        let mut unit = Source::new(&args[2], None, &unit_lines);
        unit.directives = template.directives.clone();


        template.tokenize();
        unit.tokenize();

        //let parser = Parser::new();


    } else {
        println!("source file not supplied: synth <file>")
    }
}
