use std::io::stdin;
use crate::mod_structs::*;
use crate::weapon_structs::*;

pub fn weapon_select_loop() -> GunStats {
    let mut selected = false;
    let mut gun_stats = GunStats::EMPTY_GUN;
    while !selected {
        println!("Please enter a weapon name.");
        let mut input = take_input(
            "Or, press enter for a list of supported weapons:"
        );
        gun_stats = if input.len() > 0 {
            println!("Looking up '{}'...", input);
            GunStats::gun_lookup(input.as_str())
        } else {
            println!("Using numbered table...");
            use_weapon_list()
        };
        selected = gun_stats.fire_rate != 0.0;
    };
    return gun_stats;
}

fn use_weapon_list() -> GunStats {
    println!();
    println!("Enter the number that corresponds with your weapon:");
    for (index, weapon_name) in GunStats::RIFLE_LIST.iter().enumerate() {
        println!("{}: {}", index, weapon_name)
    };
    let input = take_input("Leave blank, or fuck up the input to go back:");
    
    return if let Ok(index) = input.parse::<usize>() {
        GunStats::gun_lookup(GunStats::RIFLE_LIST[index])
    } else {
        GunStats::EMPTY_GUN
    };
}

pub fn take_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    let _ = stdin().read_line(&mut buffer);
    buffer.pop();
    return buffer;
}

// fn optimize_empty_slots(mod_list: &mut ModList, base_gun_stats: &GunStats) -> GunStats {
// }

// fn find_next_top_mod_index(mod_list: &ModList) -> i8 {
// }