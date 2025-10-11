use crate::build_calc::{SortingHelper, ModScores, GunModSums};
use crate::context_core::ModdingContext;
use crate::mod_parsing::{LoadedMods, ModData};
use crate::weapon_select::GunStats;

pub fn show_top_builds_scored(
    loaded_mods: &LoadedMods,
    sorting_helpers: &[SortingHelper],
    gun_stats: &GunStats,
    modding_context: &ModdingContext,
    count: usize,
    base_sums: Option<GunModSums>
) {
    let sums = if let Some(sums) = base_sums {
        sums
    } else {
        GunModSums::from_conditions(modding_context.conditions)
    };
    println!("The format is as follows:\nDamage\nArcane\nMod, Mod, Mod, etc...\
        \nThe (numbers) otherwise are a mod-score, higher is more impactful.\n");
    for &helper in sorting_helpers[0..count].iter() {
        let build_combo = loaded_mods.combinations[helper.index as usize];
        let arcane_name = if let Some(i) = build_combo.arcane {
            loaded_mods.get_name(i)
        } else {
            "No Arcane"
        };
        let mod_scores = ModScores::new(
            loaded_mods, gun_stats, build_combo, modding_context.damage_criteria, &sums
        );
        let arcane_score = mod_scores.arcane.unwrap_or_default();
        let scores = mod_scores.mod_scores;
        println!(
            "{}\n{} ({})\n{} ({}), {} ({}),\n{} ({}), {} ({}),\n{} ({}), {} ({}),\n{} ({}), {} ({})",
            helper.damage(),
            arcane_name,
            arcane_score,
            loaded_mods.get_name(scores[0].1),
            scores[0].0,
            loaded_mods.get_name(scores[1].1),
            scores[1].0,
            loaded_mods.get_name(scores[2].1),
            scores[2].0,
            loaded_mods.get_name(scores[3].1),
            scores[3].0,
            loaded_mods.get_name(scores[4].1),
            scores[4].0,
            loaded_mods.get_name(scores[5].1),
            scores[5].0,
            loaded_mods.get_name(scores[6].1),
            scores[6].0,
            loaded_mods.get_name(scores[7].1),
            scores[7].0,
        )
    }
}

pub fn print_riven_stats(mod_data: &ModData) {
    println!("Your riven stats are:");
    for &(stat_type, stat_value) in mod_data.get() {
        if stat_value > 0 {
            println!("+{}% {}", stat_value, stat_type.to_str());
        } else {
            println!("-{}% {}", stat_value.abs(), stat_type.to_str());
        };
    };
}

pub fn show_riven_key() {
    println!("Use the below stat-keys and a number for each value to describe your rolls:");
    println!("D: Damage");
    println!("MS: Multi-Shot");
    println!("-");
    println!("CC: CritChance");
    println!("CD: Crit Damage");
    println!("-");
    println!("C: Cold");
    println!("E: Electricity");
    println!("H: Heat");
    println!("T: Toxic");
    println!("SC: Status Chance");
    println!("-");
    println!("FR: Fire-Rate");
    println!("MC: Magazine Capacity");
    println!("RS: Reload Speed");
    println!("-");
    println!("Some examples of valid combinations:");
    println!("140 D 80 T -20 CC");
    println!("200 C -80 FR");
    println!("CC 140 CD 150 D -60");
    println!("As long as you alternate between key and values, they can be in either order.");
}