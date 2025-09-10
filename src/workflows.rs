use std::collections::HashMap;
use std::time::Instant;

use crate::context_core::{ModdingContext, WeaponType};
use crate::mod_parsing::{LoadedMods, RivenMod};
use crate::build_calc::{calculate_builds, calculate_riven_builds, get_highest_damage, SortingHelper};
use crate::cli_inputs::UserInput;
use crate::data::GUN_DATA;
use crate::display::show_top_builds;
use crate::weapon_select::{GunData, GunStats, weapon_select};

pub fn cli_workflow_entry() {
    if let Some(gun_data) = weapon_select() {
        let modding_context = ModdingContext::interview_user(
            gun_data.gun_type, gun_data.semi);
        if modding_context.riven {
            riven_input_loop(gun_data, modding_context);
        } else {
            let loaded_mods = LoadedMods::new(&modding_context);
            let build_scores = calculate_builds(
                &loaded_mods, &gun_data.gun_stats, &modding_context, None);
            let count = UserInput::looped_integer_prompt(
                "Done! How many results do you want to see? Press enter to show 6.",
                1, build_scores.len(), 6);
            show_top_builds(&loaded_mods, &build_scores, count);
        };
    } else {
        test_all_weapons();
    };
}

fn test_all_weapons() {
    let mut modding_context = ModdingContext::interview_user(
        WeaponType::Riven, false);
    let mut loaded_hashmap: HashMap<ModdingContext, LoadedMods> = HashMap::with_capacity(6);
    let mut csv_lines= GUN_DATA.lines();
    let mut gun_scores: Vec<(&str, &str, u32)> = Vec::with_capacity(539);
    let start = Instant::now();
    let mut i = 1u16;
    csv_lines.next();
    loop {
        let gun_data = if let Some(line) = csv_lines.next() {
            GunData::from_csv_line(line)
        } else {
            break;
        };
        modding_context.weapon_type = gun_data.gun_type;
        modding_context.semi = gun_data.semi;
        let loaded_mods = if let Some(result) = loaded_hashmap.get(&modding_context) {
            result
        } else {
            let result = LoadedMods::new(&modding_context);
            loaded_hashmap.insert(modding_context.clone(), result);
            loaded_hashmap.get(&modding_context).unwrap()
        };
        let gun_score = get_highest_damage(
            loaded_mods, &gun_data.gun_stats, &modding_context, None
        );
        if let Some(score) = gun_score {
            gun_scores.push((gun_data.name, gun_data.fire_mode, score));
            println!(
                "{}; {} ({}/{}) {:?} elapsed",
                gun_data.name, gun_data.fire_mode, i, gun_scores.capacity(), start.elapsed()
            );
            i+=1;
        };
    };
    gun_scores.sort_by_key(|&(name, mode, damage)| u32::MAX - damage);
    for (i, &(name, mode, damage)) in gun_scores.iter().enumerate() {
        println!("{}. {}: {}; {}", i+1, damage, name, mode);
    };
}

enum PromptChoice {
    Neutral,
    Parsed,
    Results
} impl PromptChoice {
    fn str(&self) -> & str {
        match self {
            PromptChoice::Neutral => {
                "Enter your riven mod's stats, or press enter to see the keys again."
            },
            PromptChoice::Parsed => {
                "If these stats are not correct, enter new ones now, or press enter to calculate builds"
            },
            PromptChoice::Results => {
                "Enter a number to see that many top builds. Enter new stats to score a different mod roll. Press enter to see the key again."
            }
        }
    }
}

fn riven_input_loop(gun_data: GunData, modding_context: ModdingContext) {
    let loaded_mods = LoadedMods::new(&modding_context);
    let mut builds_option: Option<Vec<SortingHelper>> = None;
    let mut reference_option: Option<f32> = None;
    let mut riven_option: Option<RivenMod> = None;
    let mut prompt_choice = PromptChoice::Neutral;
    RivenMod::show_riven_key();
    loop {
        let input_option = UserInput::new(prompt_choice.str());
        match input_option {
            Some(UserInput::Full(s)) => {
                if let Some(riven_mod) = RivenMod::from_str(&s) {
                    riven_mod.println_stats();
                    riven_option = Some(riven_mod);
                    prompt_choice = PromptChoice::Parsed;
                } else {

                }
            },
            Some(UserInput::Digit(n)) => {
                if let Some(builds) = &builds_option {
                    show_top_builds(&loaded_mods, builds, n)
                } else {
                    RivenMod::show_riven_key()
                }
            },
            _ => {
                if let Some(riven) = &riven_option {
                    let builds = calculate_riven_builds(
                        &loaded_mods, &gun_data.gun_stats, &modding_context, riven);
                    riven_option = None;

                    let reference_score = if let Some(reference_score) = reference_option {
                        reference_score
                    } else {
                        let score = generate_reference_score(&modding_context, &gun_data.gun_stats);
                        reference_option = Some(score);
                        score
                    };

                    let riven_score = get_riven_score(&builds, reference_score);
                    builds_option = Some(builds);
                    prompt_choice = PromptChoice::Results;
                    println!("Your riven score is: {}", riven_score);
                } else {
                    RivenMod::show_riven_key();
                };
            }
        };
    };
}

fn generate_reference_score(modding_context: &ModdingContext, gun_data: &GunStats) -> f32 {
    let mut reference_context = modding_context.clone();
    reference_context.riven = false;
    let reference_mods = LoadedMods::new(&reference_context);
    let score = get_highest_damage(&reference_mods, gun_data, &modding_context, None);
    if let Some(i) = score {
        i as f32
    } else {
        println!("Something went wrong with the riven score reference, do not trust your score!");
        1.0
    }
}

fn get_riven_score(builds: &Vec<SortingHelper>, reference_score: f32) -> i32 {
    let damage = (u32::MAX - builds[0].inverse_damage) as f32;
    ((damage / reference_score -1.0) * 1000.0).round() as i32
}
