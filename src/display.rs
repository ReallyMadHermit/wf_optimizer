use crate::build_calc::SortingHelper;
use crate::combinatorics::BuildCombo;
use crate::mod_parsing::LoadedMods;

// TODO: make a better display function
pub fn show_top_10(loaded_mods: &LoadedMods, sorting_helpers: Vec<SortingHelper>) {
    for n in 0..10usize {
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