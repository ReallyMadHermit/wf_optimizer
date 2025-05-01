use std::error::Error;
use std::fs::File;

use crate::weapon_structs::GunType;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WeaponMod {
    pub name: &'static str,
    pub mod_stats: [ModStat; 2]
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
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
    GalvanizedAptitude
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
    
    pub fn load_mods(gun_type: &GunType) -> Vec<WeaponMod> {
        let mut buffer = String::new();
        match gun_type {
            GunType::Rifle => {
                Self::load_rifle_mod_data(&mut buffer);
            }
        };
    }
    
    fn load_rifle_mod_data(buffer: &mut String) {
        if let Ok(csv_text) = std::fs::read_to_string("rifle_mods.csv") {
            buffer.push_str(&csv_text);
        } else {
            println!("oopsie, the rifle mods data could not be loaded, vewy sowwy, time to panic!");
            panic!();
        };
    }
    
}

pub struct WeaponArcanes;
impl WeaponArcanes {

    pub const RIFLE_ARCANE_COUNT: usize = 4;
    pub const RIFLE_ARCANES: [WeaponMod; WeaponArcanes::RIFLE_ARCANE_COUNT] = [
        WeaponMod {
            name: "Steel Path Arcane",
            mod_stats: [
                ModStat{stat_type: GunStatType::DamageOnKill, stat_value: 360},
                ModStat::empty()
            ]
        },
        WeaponMod {
            name: "Primary Frostbite",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 120},
                ModStat{stat_type: GunStatType::Multishot, stat_value: 90}
            ]
        },
        WeaponMod {
            name: "Primary Blight",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 144},
                ModStat{stat_type: GunStatType::Multishot, stat_value: 72}
            ]
        },
        WeaponMod {
            name: "Primary Crux",
            mod_stats: [
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 300},
                ModStat{stat_type: GunStatType::AmmoEfficiency, stat_value: 60}
            ]
        }
    ];

    pub const SHOTGUN_ARCANE_COUNT: usize = 5;
    pub const SHOTGUN_ARCANES: [WeaponMod; WeaponArcanes::SHOTGUN_ARCANE_COUNT] = [
        WeaponMod {
            name: "Steel Path Arcane",
            mod_stats: [
                ModStat{stat_type: GunStatType::DamageOnKill, stat_value: 360},
                ModStat{stat_type: GunStatType::ReloadSpeed, stat_value: 30}
            ]
        },
        WeaponMod {
            name: "Primary Frostbite",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 120},
                ModStat{stat_type: GunStatType::Multishot, stat_value: 90}
            ]
        },
        WeaponMod {
            name: "Primary Blight",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 144},
                ModStat{stat_type: GunStatType::Multishot, stat_value: 72}
            ]
        },
        WeaponMod {
            name: "Primary Crux",
            mod_stats: [
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 300},
                ModStat{stat_type: GunStatType::AmmoEfficiency, stat_value: 60}
            ]
        },
        WeaponMod {
            name: "Shotgun Vendetta",
            mod_stats: [
                ModStat{stat_type: GunStatType::MultishotOnKill, stat_value: 180},
                ModStat{stat_type: GunStatType::ReloadSpeedOnKill, stat_value: 75}
            ]
        }
    ];

}

pub struct RifleMods;
impl RifleMods {

    pub const MOD_COUNT: usize = 30;

    pub const ALL_MODS: [WeaponMod; RifleMods::MOD_COUNT] = [
        WeaponMod {  // 0
            name: "Amalgam Serration",
            mod_stats: [
                ModStat{stat_type: GunStatType::Damage, stat_value: 155},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 1
            name: "Argon Scope",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritChance, stat_value: 135},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 2
            name: "Bladed Rounds",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 0},
                ModStat{stat_type: GunStatType::CritDamageOnKill, stat_value: 120}
            ]
        },
        WeaponMod {  // 3
            name: "Critical Delay",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritChance, stat_value: 200},
                ModStat{stat_type: GunStatType::FireRate, stat_value: -20}
            ]
        },
        WeaponMod {  // 4
            name: "Galvanized Aptitude",
            mod_stats: [
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 80},
                ModStat{stat_type: GunStatType::ConditionOverload, stat_value: 80}
            ]
        },
        WeaponMod {  // 5
            name: "Galvanized Chamber",
            mod_stats: [
                ModStat{stat_type: GunStatType::Multishot, stat_value: 80},
                ModStat{stat_type: GunStatType::MultishotOnKill, stat_value: 150}
            ]
        },
        WeaponMod {  // 6
            name: "Galvanized Scope",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritChance, stat_value: 120},
                ModStat{stat_type: GunStatType::CritChanceOnKill, stat_value: 200}
            ]
        },
        WeaponMod {  // 7
            name: "Hammer Shot",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 60},
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 80}
            ]
        },
        WeaponMod {  // 8
            name: "Heavy Caliber",
            mod_stats: [
                ModStat{stat_type: GunStatType::Damage, stat_value: 165},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 9
            name: "Hellfire",
            mod_stats: [
                ModStat{stat_type: GunStatType::Heat, stat_value: 110},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 10
            name: "High Voltage",
            mod_stats: [
                ModStat{stat_type: GunStatType::Shock, stat_value: 60},
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 11
            name: "Infected Clip",
            mod_stats: [
                ModStat{stat_type: GunStatType::Toxic, stat_value: 110},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 12
            name: "Magnetic Capacity",
            mod_stats: [
                ModStat{stat_type: GunStatType::Magnetic, stat_value: 60},
                ModStat{stat_type: GunStatType::MagazineCapacity, stat_value: 40}
            ]
        },
        WeaponMod {  // 13
            name: "Malignant Force",
            mod_stats: [
                ModStat{stat_type: GunStatType::Toxic, stat_value: 60},
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 14
            name: "Point Strike",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritChance, stat_value: 150},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 15
            name: "Primary Acuity",
            mod_stats: [ModStat{stat_type: GunStatType::AcuityBonus, stat_value: 350},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 16
            name: "Primed Cryo Rounds",
            mod_stats: [
                ModStat {stat_type: GunStatType::Cold, stat_value: 165},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 17
            name: "Primed Shred",
            mod_stats: [
                ModStat{stat_type: GunStatType::FireRate, stat_value: 55},
                ModStat{stat_type: GunStatType::PunchThrough, stat_value: 12}
            ]
        },
        WeaponMod {  // 18
            name: "Radiated Reload",
            mod_stats: [
                ModStat{stat_type: GunStatType::Radiation, stat_value: 60},
                ModStat{stat_type: GunStatType::ReloadSpeed, stat_value: 60}
            ]
        },
        WeaponMod {  // 19
            name: "Rifle Elementalist",
            mod_stats: [
                ModStat {stat_type: GunStatType::StatusDamage, stat_value: 90},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 20
            name: "Rime Rounds",
            mod_stats: [
                ModStat{stat_type: GunStatType::Cold, stat_value: 60},
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 21
            name: "Semi-Rifle Cannonade",
            mod_stats: [
                ModStat{stat_type: GunStatType::Damage, stat_value: 0},
                ModStat{stat_type: GunStatType::DamageForSemiAuto, stat_value: 240}
            ]
        },
        WeaponMod {  // 22
            name: "Speed Trigger",
            mod_stats: [
                ModStat {stat_type: GunStatType::FireRate, stat_value: 60},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 23
            name: "Stormbringer",
            mod_stats: [
                ModStat{stat_type: GunStatType::Shock, stat_value: 110},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 24
            name: "Thermite Rounds",
            mod_stats: [
                ModStat{stat_type: GunStatType::Heat, stat_value: 60},
                ModStat{stat_type: GunStatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 25
            name: "Vile Acceleration",
            mod_stats: [
                ModStat{stat_type: GunStatType::FireRate, stat_value: 90},
                ModStat{stat_type: GunStatType::Damage, stat_value: -15}
            ]
        },
        WeaponMod {  // 26
            name: "Vital Sense",
            mod_stats: [
                ModStat{stat_type: GunStatType::CritDamage, stat_value: 120},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 27
            name: "Wildfire",
            mod_stats: [
                ModStat{stat_type: GunStatType::Heat, stat_value: 60},
                ModStat{stat_type: GunStatType::MagazineCapacity, stat_value: 20}
            ]
        },
        WeaponMod {  // 28
            name: "Vigilante Armaments",
            mod_stats: [
                ModStat{stat_type: GunStatType::Multishot, stat_value: 60},
                ModStat::empty()
            ]
        },
        WeaponMod {  // 29
            name: "Vigilante Fervor",
            mod_stats: [
                ModStat{stat_type: GunStatType::FireRate, stat_value: 45},
                ModStat::empty()
            ]
        }
    ];

    pub const CANNONADE_MODS_INDEX: [i8; 1] = [21];
    pub const DAMAGE_MODS_INDEX: [i8; 2] = [0, 8];
    pub const ELE_DAMAGE_MODS_INDEX: [i8; 4] = [16, 11, 9, 23];  // cold, toxic, heat, shock
    pub const MULTISHOT_MODS_INDEX: [i8; 2] = [5, 28];
    pub const CRIT_CHANCE_MODS_INDEX: [i8; 2] = [6, 3];
    pub const CRIT_DAMAGE_MODS_INDEX: [i8; 3] = [26, 2, 7];
    pub const FIRE_RATE_MODS_INDEX: [i8; 4] = [25, 17, 22, 29];
    
    pub const MOD_INDEXES_REFERENCES: [&'static [i8]; 7] = [
        &RifleMods::CANNONADE_MODS_INDEX,
        &RifleMods::DAMAGE_MODS_INDEX,
        &RifleMods::ELE_DAMAGE_MODS_INDEX,
        &RifleMods::MULTISHOT_MODS_INDEX,
        &RifleMods::CRIT_CHANCE_MODS_INDEX,
        &RifleMods::CRIT_DAMAGE_MODS_INDEX,
        &RifleMods::FIRE_RATE_MODS_INDEX
    ];

}

pub fn lookup_mod(gun_type: &GunType, mod_index: usize) -> &WeaponMod {
    return match gun_type {
        GunType::Rifle => {
             &RifleMods::ALL_MODS[mod_index]
        }
    };
}