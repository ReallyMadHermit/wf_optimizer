use std::env;
use std::fs;

mod json_parser;
use json_parser::*;
mod mod_structs;

fn main() {
    let contents = fs::read_to_string("mod_data.json").expect("errrr, oops");
    let lex = lexer(&contents);
}
