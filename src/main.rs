use rayon;

use std::time::Instant;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
mod brute_force_solution;

use supporting_functions::{weapon_select_loop};
use crate::weapon_structs::{Criteria};
use crate::brute_force_solution::{
    generate_combinations, filter_combinations, test_all_builds, sort_by_criteria
};
use crate::mod_structs::DataLoader;

const TOP_BUILD_COUNT: usize = 20;

fn main() {
    debug_prompts();
}

fn debug_prompts() {
    let base_weapon_stats = weapon_select_loop();
    let criteria = Criteria::determine_criteria();

    let start = Instant::now();

    let parsed_mod_list = {
        let mut buffer = String::new();
        DataLoader::load_mods(&base_weapon_stats.gun_type, &mut buffer, false)
    };
    let parsed_arcane_list = {
        let mut buffer = String::new();
        DataLoader::load_mods(&base_weapon_stats.gun_type, &mut buffer, true)
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
    
    println!("Calculating build reports...");
    let mut build_reports = test_all_builds(
        &combinations,
        &base_weapon_stats,
        &criteria,
        &parsed_mod_list,
        &parsed_arcane_list
    );
    println!("Sorting reports...");
    
    sort_by_criteria(&mut build_reports, criteria.clone());

    let duration = start.elapsed();
    println!("All done! displaying reports");
    
    println!("Elapsed: {:?}", duration);

    for i in 0..TOP_BUILD_COUNT {
        println!("{}", build_reports[i].get_report_string(&parsed_mod_list, &parsed_arcane_list))
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