use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::Arc;

mod tokenizer;
mod parser;

use tokenizer::tokenizer::*;
use parser::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if !Path::new(&args[1]).exists() {
            panic!("no such file: {}", &args[1]);
        }
        panic!("specify source files after template");

    } else if args.len() > 2 {
        println!("== started ==");

        // template -------------------------------------
        println!("\n== template ==");
        let t_f: File = match File::open(&args[1]) {
            Ok(v) => v,
            Err(_) => panic!("no such file: {}", &args[1]),
        };

        let t_raw  = BufReader::new(&t_f);
        let t_lines = t_raw.lines().map(|x| x.unwrap()).collect();

        let mut t_src = Source::new(&args[1], Some("!/def/"), &t_lines);
        t_src.tokenize();

        let mut t = template::Template::new(&t_src);
        t.parse();

        let t_arc = Arc::new(t);


        // units ----------------------------------------
        println!("\n== units ==");

        let mut unit_iter = args.iter();
        unit_iter.nth(1);

        for unit in unit_iter {
            println!("\n== unit {} ==", &unit);

            let u_f: File = match File::open(&unit) {
                Ok(v) => v,
                Err(_) => panic!("no such file: {}", &unit),
            };

            let u_raw  = BufReader::new(&u_f);
            let u_lines = u_raw.lines().map(|x| x.unwrap()).collect();

            let mut u_src = Source::new(unit, None, &u_lines);
            u_src.directives = t_arc.source.directives.clone();
            u_src.tokenize();

            let mut u = unit::Unit::new(&u_src, t_arc.clone());
            u.parse();
        }

    } else {
        println!("source file not supplied: synth <file>")
    }

    println!("\n== finished ==");
}
