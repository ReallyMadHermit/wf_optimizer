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
    // mirror_mirror_on_the_wall(GunType::Rifle);
    debug_prompts();
}

fn debug_prompts() {
    let (
        selected_gun, modding_criteria
    ) = establish_the_facts();
    
    let loaded_mods = load_mods(
        &selected_gun.gun_type,
        false
    );

    let loaded_arcanes = load_mods(
        &selected_gun.gun_type,
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
        &selected_gun.gun_stats,
        modding_criteria.damage.clone(),
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
                &selected_gun.gun_stats,
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

// fn mirror_mirror_on_the_wall(gun_type: GunType) {
//     println!("Mirror mirror on the wall, which gun is the strongest of them all?");
//     let mut buffer = String::new();
//     let data = DataLoader::new(gun_type.clone(), &mut buffer);
//     let mut modding_criteria = GunModdingContext::interview_user(gun_type.clone(), false);
//     let all_combinations = generate_combinations(data.mod_list.len() as u8);
//     let mut top_builds: Vec<(usize, u32)> = Vec::with_capacity(data.weapon_list.len());
//     let mut p = 0;
// 
//     let start = Instant::now();
//     
//     for (i, imported_gun) in data.weapon_list.iter().enumerate() {
//         p += 1;
//         println!("Calculating '{}' ({}/{})", imported_gun.get_name(), p, data.weapon_list.len());
//         modding_criteria.semi = imported_gun.get_semi();
//         let base_stats = imported_gun.get_gunstats();
//         let (required_mods, disallowed_mods) = modding_criteria.generate_filters();
//         let mut filtered_combinations = all_combinations.clone();
//         filter_combinations(&mut filtered_combinations, required_mods.as_slice(), disallowed_mods.as_slice());
//         filtered_combinations.shrink_to_fit();
//         let mut build_reports = test_all_builds(
//             &filtered_combinations,
//             &base_stats,
//             modding_criteria.damage.clone(),
//             &data.mod_list,
//             &data.arcane_list,
//         );
//         build_reports.sort_by_key(|r|r.criteria_result);
//         let t = (
//             i, 
//             build_reports[0].criteria_result.clone()
//         );
//         top_builds.push(t);
//         let mid = start.elapsed();
//         println!("Elapsed: {:?}", mid);
//     };
//     top_builds.sort_by_key(|(i, d)| *d);
//     let duration = start.elapsed();
//     println!("All done! Elapsed: {:?}", duration);
//     for (index, (i, d)) in top_builds.iter().enumerate() {
//         let dd = u32::MAX - d;
//         let ii = index + 1;
//         let n = data.weapon_list[*i].get_name();
//         let a = data.weapon_list[*i].get_attack();
//         let s = format!("{}. {} - {}: {}", ii, n, a, dd);
//         println!("{}", s);
//     };
// }

// fn cli() {
//     let base_weapon_stats = weapon_select_loop();
//     let criteria = Criteria::determine_criteria();
//     let mut mod_list = ModList::new(criteria);
//     let initial_score = fill_empty_mod_slots(&mut mod_list, &base_weapon_stats);
//     println!("Score: {}", initial_score);
// }