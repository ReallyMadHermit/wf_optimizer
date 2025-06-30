use std::path::Path;
use std::collections::VecDeque;

use crate::mod_structs::{GunStatType, LoadedGunMods};
use crate::weapon_structs::GunType;
use crate::gun_core::GunModdingContext;

pub fn read_csv(buffer: &mut String, file_name: &str) {
    let full_path = Path::new("game_data").join(file_name);
    if let Ok(csv_text) = std::fs::read_to_string(full_path) {
        buffer.push_str(&csv_text);
    } else {
        println!("oopsie, {} could not be loaded, vewy sowwy, time to panic!", file_name);
        panic!();
    };
}

pub fn load_mods(modding_context: &GunModdingContext, arcanes: bool) -> LoadedGunMods {
    let mut buffer = String::new();
        if arcanes {
            read_csv(&mut buffer, "gun_arcanes.csv");
        } else {
            read_csv(&mut buffer, "gun_mods.csv");
        };
    let mut csv_lines: VecDeque<&str> = buffer.lines().collect();
    csv_lines.pop_front();
    let mut loaded_mods = LoadedGunMods::new(csv_lines.len());
    for line in csv_lines {
        parse_gun_mod(line, modding_context, &mut loaded_mods);
    };
    return loaded_mods;
}

fn parse_gun_mod(csv_line: &str, modding_context: &GunModdingContext, loaded_mods: &mut LoadedGunMods) {
    let attributes: Vec<&str> = csv_line.split(",").collect();
    
    let mod_type = GunType::from_str(attributes[0]);
    if !GunType::is_compatible(modding_context.gun_type, mod_type) {
        return;
    };
    
    let inclusion_score = test_behaviors(&attributes[6..=11], modding_context);
    let mut included = false;
    if inclusion_score < 0 {
        return;
    } else if inclusion_score > 0 {
        included = true
    };
    
    let mod_name = attributes[1];
    let stat_type_1 = GunStatType::from_str(attributes[2]);
    let stat_value_1: i16 = if let Ok(parsed_value) = attributes[3].parse() {
        parsed_value
    } else {
        println!("Failed to load mod value 1 for {}", mod_name);
        0
    };
    
    let stat_type_2 = GunStatType::from_str(attributes[4]);
    let stat_value_2: i16 = if let Ok(parsed_value) = attributes[5].parse() {
        parsed_value
    } else {
        println!("Failed to load mod value 2 for {}", mod_name);
        0
    };

    
    if included {
        println!("Loading INCLUDED MOD {}, {}|{}", mod_name, stat_value_1, stat_value_2);
        loaded_mods.include_mod(loaded_mods.len() as u8);
    } else {
        println!("Loading {}, {}|{}", mod_name, stat_value_1, stat_value_2);
    };
    loaded_mods.load_mod(mod_name, stat_type_1, stat_value_1, stat_type_2, stat_value_2);
}

fn test_behaviors(behavior_slice: &[&str], modding_context: &GunModdingContext) -> i8 {
    let decisions = {
        let kills_behavior = ModBehavior::from_str(behavior_slice[0]);
        let aiming_behavior = ModBehavior::from_str(behavior_slice[1]);
        let semi_behavior = ModBehavior::from_str(behavior_slice[2]);
        let acuity_behavior = ModBehavior::from_str(behavior_slice[3]);
        let amalgam_behavior = ModBehavior::from_str(behavior_slice[4]);
        let riven_behavior = ModBehavior::from_str(behavior_slice[5]);
        [
            (kills_behavior, modding_context.kills),
            (aiming_behavior, modding_context.aiming),
            (semi_behavior, modding_context.semi),
            (acuity_behavior, modding_context.acuity),
            (amalgam_behavior, modding_context.prefer_amalgam),
            (riven_behavior, modding_context.riven)
        ]
    };
    for (behavior, truth) in decisions {
        if truth {
            match behavior {
                ModBehavior::Exclude => return -1,
                ModBehavior::Include => return 1,
                ModBehavior::Parallel => return 1,
                ModBehavior::NotParallel => return -1,
                _ => continue
            };
        } else {
            match behavior {
                ModBehavior::NotExclusive => return -1,
                ModBehavior::NotInclude => return 1,
                ModBehavior::Parallel => return -1,
                ModBehavior::NotParallel => return 1,
                _ => continue
            };
        };
    };
    0
}

enum ModBehavior {
    Exclude,
    Include,
    Parallel,
    NotExclusive,
    NotInclude,
    NotParallel,
    NothingSpecial
} impl ModBehavior {
    
    fn from_str(mod_behavior_str: &str) -> Self {
        match mod_behavior_str {
            "EXC"  => Self::Exclude,
            "INC"  => Self::Include,
            "PAR"  => Self::Parallel,
            "!EXC" => Self::NotExclusive,
            "!INC" => Self::NotExclusive,
            "!PAR" => Self::NotParallel,
            _      => Self::NothingSpecial
        }
    }
    
}
