use std::time::Instant;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
mod brute_force_solution;

use supporting_functions::{new_weapon_select, DataLoader};
use crate::weapon_structs::{Criteria, GunType};
use crate::brute_force_solution::{
    generate_combinations, filter_combinations, test_all_builds, sort_by_criteria
};

const TOP_BUILD_COUNT: usize = 20;

fn main() {
    debug_prompts();
}

fn debug_prompts() {
    
    let mut weapon_buffer = String::new();
    let data = DataLoader::new(GunType::Rifle, &mut weapon_buffer);
    let weapon_choice_index = new_weapon_select(&data.weapon_list);
    let imported_gun = &data.weapon_list[weapon_choice_index];
    let base_weapon_stats = imported_gun.get_gunstats(&data.gun_type);
    
    let criteria = Criteria::determine_criteria();
    let start = Instant::now();

    let mut combinations = generate_combinations(data.mod_list.len() as u8);
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    
    let required_mods: Vec<u8> = Vec::new();
    let disallowed_mods: Vec<u8> = Vec::new();

    println!("Filtering illegal pairs...");
    filter_combinations(&mut combinations, required_mods.as_slice(), disallowed_mods.as_slice());
    combinations.shrink_to_fit();
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    
    println!("Calculating build reports...");
    let mut build_reports = test_all_builds(
        &combinations,
        &base_weapon_stats,
        &criteria,
        &data.mod_list,
        &data.arcane_list
    );
    println!("Sorting reports...");
    
    sort_by_criteria(&mut build_reports, criteria.clone());

    let duration = start.elapsed();
    println!("All done! displaying reports");
    
    println!("Elapsed: {:?}", duration);
    
    println!("{}", imported_gun.get_name());
    for i in 0..TOP_BUILD_COUNT {
        println!("{}", build_reports[i].get_report_string(&data.mod_list, &data.arcane_list))
    };
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