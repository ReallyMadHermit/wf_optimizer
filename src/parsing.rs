use std::collections::VecDeque;
use std::path::Path;
use crate::mod_structs::{GunStatType, ModStat, WeaponMod};
use crate::weapon_structs::{GunType, ImportedGun};

pub struct DataLoader<'a> {
    pub gun_type: GunType,
    pub weapon_list: Vec<ImportedGun<'a>>,
    pub mod_list: Vec<WeaponMod>,
    pub arcane_list: Vec<WeaponMod>
}impl<'a> DataLoader<'a> {

    pub fn new(gun_type: GunType, weapon_buffer: &'a mut String) -> Self {
        let weapon_list = DataLoader::load_guns(&gun_type, weapon_buffer);
        let mod_list = DataLoader::load_mods(&gun_type, &mut String::new(), false);
        let arcane_list = DataLoader::load_mods(&gun_type, &mut String::new(), true);
        DataLoader {
            gun_type: gun_type.clone(),
            weapon_list,
            mod_list,
            arcane_list
        }
    }

    pub fn load_guns(gun_type: &GunType, buffer:  &'a mut String) -> Vec<ImportedGun<'a>> {
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
        let full_path = Path::new("game_data").join(file_name);
        if let Ok(csv_text) = std::fs::read_to_string(full_path) {
            buffer.push_str(&csv_text);
        } else {
            println!("oopsie, {} could not be loaded, vewy sowwy, time to panic!", file_name);
            panic!();
        };
    }

}