use std::io::stdin;
use crate::mod_structs::*;
use crate::weapon_structs::*;

pub fn take_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    let _ = stdin().read_line(&mut buffer);
    buffer
}

fn new_gun_build(weapon_name: &str) {
    let base_gun_stats = GunStats::gun_lookup(weapon_name);
    let mut mod_list = ModList::new();

}

// fn optimize_empty_slots(mod_list: &mut ModList, base_gun_stats: &GunStats) -> GunStats {
// }

// fn find_next_top_mod_index(mod_list: &ModList) -> i8 {
// }