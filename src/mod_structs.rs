use std::collections::VecDeque;
use std::path::Path;

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
    pub fn from_str(string_slice: &str) -> Self {
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
