// use std::time::Instant;

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
    let modding_context = context_core::ModdingContext::interview_user(
        gun_data.gun_type, gun_data.semi);
    let loaded_mods = mod_parsing::LoadedMods::new(&modding_context);
    // let now = Instant::now();
    let build_scores = build_calc::calculate_builds(
        &loaded_mods, &gun_data.gun_stats, modding_context.damage_criteria);
    // println!("calc_time: {:?}", now.elapsed());
    show_top_10(loaded_mods, build_scores);
}

fn show_mods(loaded_mods: &mod_parsing::LoadedMods) {
    let m = loaded_mods.arcane_count + loaded_mods.mod_count;
    for n in 0..m  {
        println!("{}", loaded_mods.get_name(n))
    };
}