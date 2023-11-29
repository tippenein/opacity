use std::env;
use std::fs;
use std::process;

mod codegen;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        process::exit(1);
    }

    let input_filename = &args[1];
    let input = fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    match parser::parse(&input) {
        Ok(ast) => {
            let output = codegen::generate_code(ast);
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Failed to parse input: {:?}", e);
            process::exit(1);
        }
    }
}
