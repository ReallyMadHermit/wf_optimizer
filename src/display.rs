use crate::build_calc::SortingHelper;
use crate::combinatorics::BuildCombo;
use crate::mod_parsing::{LoadedMods, ModData};

pub fn show_top_builds(loaded_mods: &LoadedMods, sorting_helpers: &Vec<SortingHelper>, count: usize) {
    for n in 0..count {
        let helper = sorting_helpers[n];
        let combo = &loaded_mods.combinations[helper.index as usize];
        display_build(&loaded_mods, combo, helper);
    };
}

fn display_build(loaded_mods: &LoadedMods, build_combo: &BuildCombo, sorting_helper: SortingHelper) {
    let arcane_name = if let Some(i) = build_combo.arcane {
        loaded_mods.get_name(i)
    } else {
        "No Arcane"
    };
    let mut mod_names = [""; 8];
    for (index, &mod_id) in build_combo.mod_combo.iter().enumerate() {
        mod_names[index] = loaded_mods.get_name(mod_id);
    };
    println!(
        "{}\n{}\n{}, {}, {}, {},\n{}, {}, {}, {}",
        sorting_helper.damage(),
        arcane_name,
        mod_names[0],
        mod_names[1],
        mod_names[2],
        mod_names[3],
        mod_names[4],
        mod_names[5],
        mod_names[6],
        mod_names[7],
    );
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