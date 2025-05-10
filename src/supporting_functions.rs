use std::collections::VecDeque;
use std::io::stdin;
use std::path::Path;
use crate::mod_structs::*;
use crate::weapon_structs::*;
use std::fmt::Write;

pub const fn build_mask(indices: &[u8]) -> u64 {
    let mut mask: u64 = 0;
    let mut i = 0;
    while i < indices.len() {
        mask |= 1 << indices[i];
        i += 1;
    };
    mask
}

pub fn new_weapon_select(imported_guns: &Vec<ImportedGun>) -> usize {
    let mut results:Vec<usize> = Vec::with_capacity(4);
    println!("Enter the weapon's name (it's case sensitive (out of spite, of course))");
    let input = take_input("Leave blank, or fuck up the input to go back:");
    for (index, gun) in imported_guns.iter().enumerate() {
        if gun.get_name() == &input {
            results.push(index)
        };
    };
    let l = results.len();
    if l < 1 {
        return new_weapon_list_select(imported_guns, &input);
    } else if l > 1 {
        return sub_weapon_select(imported_guns, &results);
    };
    results[0]
}

pub fn sub_weapon_select(imported_guns: &Vec<ImportedGun>, results: &Vec<usize>) -> usize {
    let mut buffer = String::from("There are multiple weapons with that name:");
    for (index, &result) in results.iter().enumerate() {
        let var = imported_guns[result].get_attack();
        _ = writeln!(buffer, "{}. {}", index, var);
    };
    _ = writeln!(buffer, "Please choose the variant number you wish to optimize.");
    let input = take_input(&buffer);
    return if let Ok(choice) = input.parse() {
        let choice: usize = choice;
        return results[choice]
    } else {
        println!("Something went wrong! have the first one ig");
        results[0]
    };
}

pub fn new_weapon_list_select(imported_guns: &Vec<ImportedGun>, last_input: &str) -> usize {
    let mut buffer = String::with_capacity(1200);
    let empty = last_input == "";
    if empty {
        _ = writeln!(buffer, "You didn't input anything, or did it so badly, I couldn't tell.");
    } else {
        _ = writeln!(buffer, "Right... Couldn't find that, but I narrowed the results a bit.");
    };
    let mut last_name = "";
    let input_first = last_input.to_uppercase().chars().next();
    for (index, weapon) in imported_guns.iter().enumerate() {
        let name = weapon.get_name();
        if name == last_name {  // prevents listing weapons with 2 modes twice
            continue;
        } else {
            last_name = name;
        };
        if empty || input_first == name.chars().next() {  // checks if input is empty
            _ = writeln!(buffer, "{}. {}", index, name);
        };
    };
    _ = writeln!(buffer, "Please enter the number corresponding with the weapon you want to customize...");
    let input = take_input(&buffer);
    let ii: usize = if let Ok(choice) = input.parse() {
        choice
    } else {
        println!("You've bungled it somehow... I'm just gonna give you the first one off the top.");
        0
    };
    return ii;
}

// pub fn weapon_select_loop() -> GunStats {
//     let mut selected = false;
//     let mut gun_stats = GunStats::EMPTY_GUN;
//     while !selected {
//         println!("Please enter a weapon name.");
//         let mut input = take_input(
//             "Or, press enter for a list of supported weapons:"
//         );
//         gun_stats = if input.len() > 0 {
//             println!("Looking up '{}'...", input);
//             GunStats::gun_lookup(input.as_str())
//         } else {
//             println!("Using numbered table...");
//             use_weapon_list()
//         };
//         selected = gun_stats.fire_rate != 0.0;
//     };
//     return gun_stats;
// }
//
// fn use_weapon_list() -> GunStats {
//     println!();
//     println!("Enter the number that corresponds with your weapon:");
//     for (index, weapon_name) in GunStats::RIFLE_LIST.iter().enumerate() {
//         println!("{}: {}", index, weapon_name)
//     };
//     let input = take_input("Leave blank, or fuck up the input to go back:");
//
//     return if let Ok(index) = input.parse::<usize>() {
//         GunStats::gun_lookup(GunStats::RIFLE_LIST[index])
//     } else {
//         GunStats::EMPTY_GUN
//     };
// }

pub fn take_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    let _ = stdin().read_line(&mut buffer);
    buffer.pop();
    return buffer;
}

fn try_mod(
    mod_sum: &GunStatModSums, base_stats: &GunStats, weapon_mod: &WeaponMod, criteria: &Criteria
) -> f32 {  // this should return a number representing the effective multiplier the mod applies
    let old_stats = base_stats.apply_stat_sums(&mod_sum);
    let mut new_mod_sum = mod_sum.clone();
    new_mod_sum.add_mod(weapon_mod, criteria.kills(), base_stats.semi);
    let new_stats = base_stats.apply_stat_sums(&new_mod_sum);
    return compare_stats(&old_stats, &new_stats, criteria);
}

fn compare_stats(
    old_stats: &GunStats, new_stats: &GunStats, criteria: &Criteria
) -> f32 {
    let old_shot_damage = old_stats.calculate_shot_damage();
    let new_shot_damage = new_stats.calculate_shot_damage();
    if criteria == &Criteria::PerShot || criteria == &Criteria::PerShotNoKills {
        return new_shot_damage / old_shot_damage;
    };
    let old_burst_damage = old_stats.calculate_burst_dps(old_shot_damage);
    let new_burst_damage = new_stats.calculate_burst_dps(new_shot_damage);
    if criteria == &Criteria::BurstDPS || criteria == &Criteria::BurstDPSNoKills {
        return new_burst_damage / old_burst_damage;
    };
    let old_sustained_damage = old_stats.calculate_sustained_dps(old_burst_damage);
    let new_sustained_damage = new_stats.calculate_sustained_dps(new_burst_damage);
    return new_sustained_damage / old_sustained_damage;
}

pub struct DataLoader;
impl DataLoader {

    pub fn load_guns<'a>(gun_type: &GunType, buffer:  &'a mut String) -> Vec<ImportedGun<'a>> {
        match gun_type {
            GunType::Rifle => {
                Self::read_csv(buffer, "rifle_stats.csv");
            }
        };
        let mut csv_lines: VecDeque<&str> = buffer.lines().collect();
        csv_lines.pop_front();
        let mut imported_guns: Vec<ImportedGun> = Vec::with_capacity(csv_lines.len());
        for csv_line in  csv_lines {
            imported_guns.push(
                ImportedGun::new(csv_line)
            );
        };
        return imported_guns;
    }

    pub fn load_mods(gun_type: &GunType, buffer: &mut String, arcanes: bool) -> Vec<WeaponMod> {
        match gun_type {
            GunType::Rifle => {
                if arcanes {
                    Self::read_csv(buffer, "rifle_arcanes.csv");
                } else {
                    Self::read_csv(buffer, "rifle_mods.csv");
                };
            }
        };
        let mut csv_lines: VecDeque<&str> = buffer.lines().collect();
        csv_lines.pop_front();
        let mut mod_list: Vec<WeaponMod> = Vec::with_capacity(csv_lines.len());
        for line in csv_lines {
            mod_list.push(
                DataLoader::parse_gun_mod(line)
            );
        };
        return mod_list;
    }

    fn parse_gun_mod(csv_line: &str) -> WeaponMod {
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
        WeaponMod {
            name: String::from(mod_name),
            mod_stats: [
                ModStat {
                    stat_type: stat_type_1,
                    stat_value: stat_value_1
                },
                ModStat {
                    stat_type: stat_type_2,
                    stat_value: stat_value_2
                }
            ]
        }
    }

    fn read_csv(buffer: &mut String, file_name: &str) {
        let full_path = Path::new("data").join(file_name);
        if let Ok(csv_text) = std::fs::read_to_string(full_path) {
            buffer.push_str(&csv_text);
        } else {
            println!("oopsie, {} could not be loaded, vewy sowwy, time to panic!", file_name);
            panic!();
        };
    }

}
