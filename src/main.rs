use std::time::Instant;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
mod brute_force_solution;

use supporting_functions::{weapon_select_loop};
use crate::weapon_structs::{Criteria, GunType};
use crate::brute_force_solution::{generate_combinations, filter_combinations};
use crate::mod_structs::ModLoader;

fn main() {
    debug_prompts();
}

fn debug_prompts() {
    let start = Instant::now();
    let parsed_mod_list = {
        let mut buffer = String::new();
        ModLoader::load_gun_mods(&GunType::Rifle, &mut buffer)
    };
    let mut combinations = generate_combinations(parsed_mod_list.len() as u8);
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    
    let required_mods: Vec<usize> = Vec::new();
    let disallowed_mods: Vec<usize> = Vec::new();
    println!("Filtering illegal pairs...");
    filter_combinations(&mut combinations, required_mods.as_slice(), disallowed_mods.as_slice());
    combinations.shrink_to_fit();
    
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);
}

fn print_combo(combo: &[u8; 8]) {
    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}",
        combo[0], combo[1], combo[2], combo[3], combo[4], combo[5], combo[6], combo[7],
    );
}

// fn cli() {
//     let base_weapon_stats = weapon_select_loop();
//     let criteria = Criteria::determine_criteria();
//     let mut mod_list = ModList::new(criteria);
//     let initial_score = fill_empty_mod_slots(&mut mod_list, &base_weapon_stats);
//     println!("Score: {}", initial_score);
// }