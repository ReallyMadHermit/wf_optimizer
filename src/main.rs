use std::time::{Duration, Instant};
use crate::display::show_top_10;
use crate::mod_parsing::LoadedMods;

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
    let mut load_time = Duration::default();
    let mut combo_time = Duration::default();
    let mut calc_time = Duration::default();
    let mut sort_time = Duration::default();
    let mut start = Instant::now();
    let loaded_mods = mod_parsing::LoadedMods::new(&modding_context);
    load_time = start.elapsed();
    start = Instant::now();
    let combinations = combinatorics::generate_combinations(loaded_mods.mod_count, loaded_mods.arcane_count);
    combo_time = start.elapsed();
    start = Instant::now();
    let mut builds = build_calc::calculate_builds(&combinations, &loaded_mods, &gun_data.gun_stats, modding_context.damage_criteria);
    calc_time = start.elapsed();
    start = Instant::now();
    builds.sort_by_key(|build| build.inverse_damage);  // TODO: move into the build calc, it's only here for benching
    sort_time = start.elapsed();
    let total = load_time + combo_time + calc_time + sort_time;
    println!("Combos computed: {}", combinations.len());
    println!("Mod load-time: {:?}", load_time);
    println!("Combinatorics time: {:?}", combo_time);
    println!("Calc time: {:?}", calc_time);
    println!("Sort time: {:?}", sort_time);
    println!("Total: {:?}", total);
    show_top_10(loaded_mods, combinations, builds);
}

fn show_mods(loaded_mods: &LoadedMods) {
    let m = loaded_mods.arcane_count + loaded_mods.mod_count;
    for n in 0..m  {
        println!("{}", loaded_mods.get_name(n))
    };
}