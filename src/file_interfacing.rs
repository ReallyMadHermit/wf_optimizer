use std::path::Path;
use std::collections::VecDeque;

use crate::mod_structs::{GunStatType, LoadedMods};
use crate::weapon_structs::GunType;

pub fn read_csv(buffer: &mut String, file_name: &str) {
    let full_path = Path::new("game_data").join(file_name);
    if let Ok(csv_text) = std::fs::read_to_string(full_path) {
        buffer.push_str(&csv_text);
    } else {
        println!("oopsie, {} could not be loaded, vewy sowwy, time to panic!", file_name);
        panic!();
    };
}

pub fn load_mods(gun_type: &GunType, arcanes: bool) -> LoadedMods {
    let mut buffer = String::new();
    match gun_type {
        _ => {
            if arcanes {
                read_csv(&mut buffer, "rifle_arcanes.csv");
            } else {
                read_csv(&mut buffer, "rifle_mods.csv");
            };
        }
    };
    let mut csv_lines: VecDeque<&str> = buffer.lines().collect();
    csv_lines.pop_front();
    let mut loaded_mods = LoadedMods::new(csv_lines.len());
    for line in csv_lines {
        parse_gun_mod(line, &mut loaded_mods);
    };
    return loaded_mods;
}

fn parse_gun_mod(csv_line: &str, loaded_mods: &mut LoadedMods) {
    let attributes: Vec<&str> = csv_line.split(",").collect();
    let mod_name = attributes[0];

    let stat_type_1 = GunStatType::from_str(attributes[1]);
    let stat_value_1: i16 = if let Ok(parsed_value) = attributes[2].parse() {
        parsed_value
    } else {
        println!("Failed to load mod value 1 for {}", mod_name);
        0
    };

    let stat_type_2 = GunStatType::from_str(attributes[3]);
    let stat_value_2: i16 = if let Ok(parsed_value) = attributes[4].parse() {
        parsed_value
    } else {
        println!("Failed to load mod value 2 for {}", mod_name);
        0
    };

    println!("Loading {}, {}|{}", mod_name, stat_value_1, stat_value_2);
    loaded_mods.load_mod(mod_name, stat_type_1, stat_value_1, stat_type_2, stat_value_2);
}
