use std::collections::VecDeque;

use crate::weapon_structs::GunType;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WeaponMod {
    pub name: String,
    pub mod_stats: [ModStat; 2]
}

pub struct RivenMod {
    mod_stat_array: [ModStat; 4]
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum GunStatType {
    None,
    Damage,
    DamageForSemiAuto,
    DamageOnKill,
    Heat,
    Cold,
    Toxic,
    Shock,
    Magnetic,
    Radiation,
    Multishot,
    MultishotOnKill,
    CritChance,
    CritChanceOnKill,
    CritDamage,
    CritDamageOnKill,
    FireRate,
    StatusChance,
    ConditionOverload,  // always conditional
    MagazineCapacity,
    ReloadSpeed,
    ReloadSpeedOnKill,
    AcuityBonus,  // crit chance + weak point damage
    StatusDamage,
    PunchThrough,
    AmmoEfficiency,
    Riven
} impl GunStatType {
    fn from_str(string_slice: &str) -> Self {
        return match string_slice {
            "None" => GunStatType::None,
            "Damage" => GunStatType::Damage,
            "DamageForSemiAuto" => GunStatType::DamageForSemiAuto,
            "DamageOnKill" => GunStatType::DamageOnKill,
            "Heat" => GunStatType::Heat,
            "Cold" => GunStatType::Cold,
            "Toxic" => GunStatType::Toxic,
            "Shock" => GunStatType::Shock,
            "Magnetic" => GunStatType::Magnetic,
            "Radiation" => GunStatType::Radiation,
            "Multishot" => GunStatType::Multishot,
            "MultishotOnKill" => GunStatType::MultishotOnKill,
            "CritChance" => GunStatType::CritChance,
            "CritChanceOnKill" => GunStatType::CritChanceOnKill,
            "CritDamage" => GunStatType::CritDamage,
            "CritDamageOnKill" => GunStatType::CritDamageOnKill,
            "FireRate" => GunStatType::FireRate,
            "StatusChance" => GunStatType::StatusChance,
            "ConditionOverload" => GunStatType::ConditionOverload,
            "MagazineCapacity" => GunStatType::MagazineCapacity,
            "ReloadSpeed" => GunStatType::ReloadSpeed,
            "ReloadSpeedOnKill" => GunStatType::ReloadSpeedOnKill,
            "AcuityBonus" => GunStatType::AcuityBonus,
            "StatusDamage" => GunStatType::StatusDamage,
            "PunchThrough" => GunStatType::PunchThrough,
            "AmmoEfficiency" => GunStatType::AmmoEfficiency,
            "Riven" => GunStatType::Riven,
            _ => {
                println!("{} not found! Using 'None'", string_slice);
                GunStatType::None
            }
        };
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModStat {
    pub stat_type: GunStatType,
    pub stat_value: i16
} impl ModStat {
    
    const fn new(stat_type: GunStatType, stat_value: i16) -> Self {
        ModStat {
            stat_type,
            stat_value
        }
    }
    
    const fn empty() -> Self {
        ModStat {
            stat_type: GunStatType::None,
            stat_value: 0
        }
    }
    
}

pub struct ModLoader;
impl ModLoader {

    pub fn load_mods(gun_type: &GunType, buffer: &mut String, arcanes: bool) -> Vec<WeaponMod> {
        match gun_type {
            GunType::Rifle => {
                if arcanes {
                    Self::read_csv(buffer, "rifle_arcanes.csv")
                } else {
                    Self::read_csv(buffer, "rifle_mods.csv");
                };
            }
        };
        let mut csv_lines: VecDeque<&str> = buffer.lines().collect();
        csv_lines.pop_front();
        let mut mod_list: Vec<WeaponMod> = Vec::with_capacity(csv_lines.len());
        for line in csv_lines {
            if &line[0..1] == "," {
                continue;
            };
            mod_list.push(
                ModLoader::parse_gun_mod(line)
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
        if let Ok(csv_text) = std::fs::read_to_string(file_name) {
            buffer.push_str(&csv_text);
        } else {
            println!("oopsie, {} could not be loaded, vewy sowwy, time to panic!", file_name);
            panic!();
        };
    }

}
