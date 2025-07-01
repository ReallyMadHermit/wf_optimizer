use std::time::Instant;

mod mod_structs;
mod weapon_structs;
mod brute_force_solution;
mod file_interfacing;
mod cli_inputs;
mod gun_core;

use cli_inputs::{establish_the_facts, take_input, parse_input};
use crate::brute_force_solution::{
    generate_combinations, filter_combinations, test_all_builds
};
use crate::file_interfacing::load_mods;

const TOP_BUILD_COUNT: usize = 20;

fn main() {
    compact_core();
}

fn debug_prompts() {
    let (
        selected_gun, gun_modding_context
    ) = establish_the_facts();
    
    let loaded_mods = load_mods(
        &gun_modding_context,
        false
    );

    let loaded_arcanes = load_mods(
        &gun_modding_context,
        true
    );

    let start = Instant::now();

    let mut combinations = generate_combinations(loaded_mods.len() as u8);
    let count = combinations.len();
    println!("Combinations: {}", count);
    println!("First combo:");
    print_combo(&combinations[0]);
    println!("Last combo:");
    print_combo(&combinations[count - 1]);
    
    let required_mods = loaded_mods.included_mods_slice();

    println!("Filtering illegal pairs...");
    filter_combinations(&mut combinations, required_mods);
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
        &selected_gun.gun_stats,
        gun_modding_context.damage,
        &loaded_mods,
        &loaded_arcanes,
    );
    println!("Sorting reports...");
    build_reports.sort_by_key(|r|r.criteria_result);

    let duration = start.elapsed();
    println!("All done! Elapsed: {:?}", duration);
    let display_input = take_input("How many reports should we show?");
    let report_display_count = parse_input(&display_input);
    
    println!("{}\nHit|Burst|Sustain", &selected_gun.name);
    for i in 0..report_display_count {
        println!(
            "{}",
            build_reports[i].get_report_string(
                // &selected_gun.gun_stats,
                &combinations,
                &loaded_mods,
                &loaded_arcanes
            )
        );
    };
}

fn compact_core() {
    let (
        selected_gun, gun_modding_context
    ) = establish_the_facts();
    let start = Instant::now();
    let loaded_mods = load_mods(
        &gun_modding_context,
        false
    );
    let loaded_arcanes = load_mods(
        &gun_modding_context,
        true
    );
    let mut combinations = generate_combinations(loaded_mods.len() as u8);
    let required_mods = loaded_mods.included_mods_slice();
    filter_combinations(&mut combinations, required_mods);
    combinations.shrink_to_fit();
    let mut build_reports = test_all_builds(
        &combinations,
        &selected_gun.gun_stats,
        gun_modding_context.damage,
        &loaded_mods,
        &loaded_arcanes,
    );
    build_reports.sort_by_key(|r|r.criteria_result);
    let duration = start.elapsed();
    println!("All done! Elapsed: {:?}", duration);
    let display_input = take_input("How many reports should we show?");
    let report_display_count = parse_input(&display_input);

    // println!("{}\nHit|Burst|Sustain", &selected_gun.name);
    println!("{}, {}", &selected_gun.name, gun_modding_context.damage.str());
    for i in 0..report_display_count {
        println!(
            "{}",
            build_reports[i].get_report_string(
                // &selected_gun.gun_stats,
                &combinations,
                &loaded_mods,
                &loaded_arcanes
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
