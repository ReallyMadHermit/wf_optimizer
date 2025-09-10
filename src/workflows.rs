use crate::context_core::ModdingContext;
use crate::mod_parsing::{LoadedMods, RivenMod};
use crate::weapon_select;
use crate::build_calc::{calculate_builds, calculate_riven_builds, get_highest_damage, SortingHelper};
use crate::cli_inputs::UserInput;
use crate::display::show_top_builds;
use crate::weapon_select::{GunData, GunStats};

pub fn cli_workflow_entry() {
    let gun_data = weapon_select::weapon_select();
    let modding_context = ModdingContext::interview_user(
        gun_data.gun_type, gun_data.semi);
    if modding_context.riven {
        riven_input_loop(gun_data, modding_context)
    } else {
        let loaded_mods = LoadedMods::new(&modding_context);
        let build_scores = calculate_builds(
            &loaded_mods, &gun_data.gun_stats, &modding_context, None);
        show_top_builds(&loaded_mods, &build_scores, 6);
    }
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
