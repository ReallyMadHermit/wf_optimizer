use std::io::stdin;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
use supporting_functions::weapon_select_loop;

fn main() {
    weapon_select_loop();
}