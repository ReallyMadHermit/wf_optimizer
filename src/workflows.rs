use crate::context_core::ModdingContext;
use crate::mod_parsing::{LoadedMods, RivenMod};
use crate::{build_calc, weapon_select};
use crate::build_calc::{get_highest_damage, GunModSums};
use crate::cli_inputs::UserInput;
use crate::display::show_top_10;
use crate::weapon_select::GunData;

pub fn cli_workflow_entry() {
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
    let reference_score = {
        let mut reference_context = modding_context.clone();
        reference_context.riven = false;
        let reference_mods = LoadedMods::new(&reference_context);
        let score = get_highest_damage(&reference_mods, &gun_data.gun_stats, &modding_context, None);
        if let Some(i) = score {
            i as f32
        } else {
            println!("Something went wrong with the riven score reference, do not trust your score!");
            1.0
        }
    };
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
        let top_score = (u32::MAX - build_scores.first().unwrap().inverse_damage) as f32;
        let riven_score = (((top_score / reference_score) - 1.0) * 1000.0).round() as i32;
        println!("Riven score: {}", riven_score);
        show_top_10(&loaded_mods, build_scores);
    }
}