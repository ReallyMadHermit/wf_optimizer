mod mod_structs;
mod weapon_structs;
mod supporting_functions;
mod brute_force_solution;

use supporting_functions::{weapon_select_loop};
use crate::supporting_functions::fill_empty_mod_slots;
use crate::weapon_structs::{ModList, Criteria};
use crate::brute_force_solution::generate_combinations;

fn main() {
    let combinations = generate_combinations(32);
    println!("{}", combinations.len());
    print_combo(&combinations[0]);
}

fn print_combo(combo: &[u8; 8]) {
    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}",
        combo[0], combo[1], combo[2], combo[3], combo[4], combo[5], combo[6], combo[7],
    );
}

fn cli() {
    let base_weapon_stats = weapon_select_loop();
    let criteria = Criteria::determine_criteria();
    let mut mod_list = ModList::new(criteria);
    let initial_score = fill_empty_mod_slots(&mut mod_list, &base_weapon_stats);
    println!("Score: {}", initial_score);
}