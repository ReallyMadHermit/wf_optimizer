use std::io::stdin;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
use supporting_functions::take_input;

fn main() {
    let input = take_input("give me your input");
    println!("the input taken: {}", input)
}
