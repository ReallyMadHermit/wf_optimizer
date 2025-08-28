// mod mod_structs;
// mod weapon_structs;
// mod brute_force_solution;
// mod file_interfacing;
// mod cli_inputs;
// mod gun_core;

// pub mod old;
//
// use old::cli_inputs::cli_build_calculation_workflow;
//
// const TOP_BUILD_COUNT: usize = 20;

use std::time::{Duration, Instant};
use crate::display::show_top_10;

mod data;
mod combinatorics;
mod cli_inputs;
mod mod_parsing;
mod weapon_select;
mod context_core;
mod build_calc;
mod display;

fn main() {
    workflow()
}

fn workflow() {
    let gun_data = weapon_select::weapon_select();
    let modding_context = context_core::ModdingContext::interview_user(gun_data.gun_type, gun_data.semi);
    let loaded_mods = mod_parsing::LoadedMods::new(&modding_context);
    let combinations = combinatorics::generate_combinations(loaded_mods.mod_count, loaded_mods.arcane_count);
    let builds = build_calc::calculate_builds(&combinations, &loaded_mods, &gun_data.gun_stats, modding_context.damage_criteria);
    show_top_10(loaded_mods, combinations, builds);
}