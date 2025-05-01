mod mod_structs;
mod weapon_structs;
mod supporting_functions;
mod brute_force_solution;

use supporting_functions::{weapon_select_loop};
use crate::weapon_structs::{ModList, Criteria, GunType};
use crate::brute_force_solution::generate_combinations;
use crate::mod_structs::ModLoader;

fn main() {
    // let combinations = generate_combinations(32);
    // let count = combinations.len();
    // println!("{}", count);
    // print_combo(&combinations[0]);
    // print_combo(&combinations[count - 1]);
    let mut buffer = String::new();
    let parsed_mods = ModLoader::load_gun_mods(&GunType::Rifle, &mut buffer);
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