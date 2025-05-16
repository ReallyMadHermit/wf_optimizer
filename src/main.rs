use std::time::Instant;
use std::cmp::Reverse;

mod mod_structs;
mod weapon_structs;
mod supporting_functions;
mod brute_force_solution;

use supporting_functions::establish_the_facts;
use crate::brute_force_solution::{
    generate_combinations, filter_combinations, test_all_builds
};
use crate::supporting_functions::{parse_input, take_input};

const TOP_BUILD_COUNT: usize = 20;

fn main() {
    debug_prompts();
}

fn debug_prompts() {

    let mut weapon_buffer = String::new();
    let (
        data, weapon_choice_index, modding_criteria
    ) = establish_the_facts(&mut weapon_buffer);

    let imported_gun = &data.weapon_list[weapon_choice_index];
    let base_weapon_stats = imported_gun.get_gunstats();

    let start = Instant::now();

    let mut combinations = generate_combinations(data.mod_list.len() as u8);
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    
    let (required_mods, disallowed_mods) = modding_criteria.generate_filters();
    // let required_mods = Vec::new();
    // let disallowed_mods = Vec::new();

    println!("Filtering illegal pairs...");
    filter_combinations(&mut combinations, required_mods.as_slice(), disallowed_mods.as_slice());
    combinations.shrink_to_fit();
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    
    println!("Calculating builds...");
    let mut build_reports = test_all_builds(
        &combinations,
        &base_weapon_stats,
        modding_criteria.damage.clone(),
        &data.mod_list,
        &data.arcane_list,
    );
    println!("Sorting reports...");
    build_reports.sort_by_key(|r|r.criteria_result);

    let duration = start.elapsed();
    println!("All done! Elapsed: {:?}", duration);
    let display_input = take_input("How many reports should we show?");
    let report_display_count = parse_input(&display_input);
    
    println!("{}\nHit|Burst|Sustain", imported_gun.get_name());
    for i in 0..report_display_count {
        println!(
            "{}",
            build_reports[i].get_report_string(
                &base_weapon_stats,
                &combinations,
                &data.mod_list,
                &data.arcane_list
            )
        );
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