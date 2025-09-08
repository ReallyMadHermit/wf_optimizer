// use std::time::Instant;

use crate::build_calc::GunModSums;
use crate::cli_inputs::UserInput;
use crate::context_core::ModdingContext;
use crate::display::show_top_10;
use crate::mod_parsing::{LoadedMods, RivenMod};
use crate::weapon_select::GunData;

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

// let now = Instant::now();
// println!("calc_time: {:?}", now.elapsed());

fn workflow() {
    let gun_data = weapon_select::weapon_select();
    let modding_context = ModdingContext::interview_user(
        gun_data.gun_type, gun_data.semi);
    let loaded_mods = LoadedMods::new(&modding_context);
    if modding_context.riven {
        riven_loop(gun_data, modding_context, loaded_mods)
    } else {
        let build_scores = build_calc::calculate_builds(
            &loaded_mods, &gun_data.gun_stats, &modding_context, None);
        show_top_10(&loaded_mods, build_scores);
    }
}

fn riven_loop(gun_data: GunData, modding_context: ModdingContext, loaded_mods: LoadedMods) {
    let mut running = true;
    while running {
        let s: String;
        let u = UserInput::riven_prompt();
        if let Some(UserInput::Full(full_string)) = u {
            s = full_string
        } else {
            println!("That wasn't right, try again.");
            continue;
        };
        let riven_parsed = RivenMod::from_str(&s);
        let riven_mod = if let Some(riven_mod) = riven_parsed {
            println!("Just to verify your input, this is roughly what you meant...");
            riven_mod.println_stats();
            let c_prompt = UserInput::yes_no_prompt("Continue with these stats", true);
            if !c_prompt {  // TODO loop the input instead of a Y/N here
                continue;
            };
            riven_mod
        } else {
            println!("That wasn't right, try again.");
            continue;
        };
        let build_scores = build_calc::calculate_builds(
            &loaded_mods, &gun_data.gun_stats, &modding_context, Some(GunModSums::from_riven(&riven_mod)));
        show_top_10(&loaded_mods, build_scores);
    }
}

// fn show_mods(loaded_mods: &mod_parsing::LoadedMods) {
//     let m = loaded_mods.arcane_count + loaded_mods.mod_count;
//     for n in 0..m  {
//         println!("{}", loaded_mods.get_name(n))
//     };
// }