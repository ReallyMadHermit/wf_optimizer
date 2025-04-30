use std::io::stdin;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
use supporting_functions::{weapon_select_loop};
use crate::supporting_functions::fill_empty_mod_slots;
use crate::weapon_structs::{ModList, Criteria};

fn main() {
    cli();
}

fn cli() {
    let base_weapon_stats = weapon_select_loop();
    let criteria = Criteria::determine_criteria();
    let mut mod_list = ModList::new(criteria);
    let initial_score = fill_empty_mod_slots(&mut mod_list, &base_weapon_stats);
    println!("Score: {}", initial_score);
}