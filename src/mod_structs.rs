#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WeaponMod {
    pub name: String,
    pub stats: Vec<ModStat>
}
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum StatType {
    None,
    Damage,
    Heat,
    Cold,
    Toxic,
    Shock,
    Magnetic,
    Radiation,
    Multishot,
    CritChance,
    CritDamage,
    FireRate,
    StatusChance,
    ConditionOverload,  // always conditional
    CritChanceOnKill,
    CritDamageOnKill,
    MultishotOnKill,
    MagazineCapacity,
    ReloadSpeed,
    AcuityBonus,  // crit chance + weak point damage
    StatusDamage,
    PunchThrough,
    SemiAutoCondition
}
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModStat {
    pub stat_type: StatType,
    pub stat_value: i16
}

pub struct RifleMods;
impl RifleMods {
    pub const MOD_COUNT: usize = 30;
    pub const ALL_MODS: [WeaponMod; RifleMods::MOD_COUNT] = [
        WeaponMod {  // 0
            name: "Amalgam Serration".into_string(),
            stats: vec![ModStat{stat_type: StatType::Damage, stat_value: 155}]
        },
        WeaponMod {  // 1
            name: "Argon Scope".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritChance, stat_value: 135}]
        },
        WeaponMod {  // 2
            name: "Bladed Rounds".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::CritDamage, stat_value: 0},
                ModStat{stat_type: StatType::CritDamageOnKill, stat_value: 120}
            ]
        },
        WeaponMod {  // 3
            name: "Critical Delay".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::CritChance, stat_value: 200},
                ModStat{stat_type: StatType::FireRate, stat_value: -20}
            ]
        },
        WeaponMod {  // 4
            name: "Galvanized Aptitude".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::StatusChance, stat_value: 80},
                ModStat{stat_type: StatType::ConditionOverload, stat_value: 80}
            ]
        },
        WeaponMod {  // 5
            name: "Galvanized Chamber".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Multishot, stat_value: 80},
                ModStat{stat_type: StatType::MultishotOnKill, stat_value: 150}
            ]
        },
        WeaponMod {  // 6
            name: "Galvanized Scope".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::CritChance, stat_value: 120},
                ModStat{stat_type: StatType::CritChanceOnKill, stat_value: 200}
            ]
        },
        WeaponMod {  // 7
            name: "Hammer Shot".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::CritDamage, stat_value: 60},
                ModStat{stat_type: StatType::StatusChance, stat_value: 80}
            ]
        },
        WeaponMod {  // 8
            name: "Heavy Caliber".into_string(),
            stats: vec![ModStat{stat_type: StatType::Damage, stat_value: 165}]
        },
        WeaponMod {  // 9
            name: "Hellfire".into_string(),
            stats: vec![ModStat{stat_type: StatType::Heat, stat_value: 110}]
        },
        WeaponMod {  // 10
            name: "High Voltage".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Shock, stat_value: 60},
                ModStat{stat_type: StatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 11
            name: "Infected Clip".into_string(),
            stats: vec![ModStat{stat_type: StatType::Toxic, stat_value: 110}]
        },
        WeaponMod {  // 12
            name: "Magnetic Capacity".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Magnetic, stat_value: 60},
                ModStat{stat_type: StatType::MagazineCapacity, stat_value: 40}
            ]
        },
        WeaponMod {  // 13
            name: "Malignant Force".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Toxic, stat_value: 60},
                ModStat{stat_type: StatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 14
            name: "Point Strike".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritChance, stat_value: 150}]
        },
        WeaponMod {  // 15
            name: "Primary Acuity".into_string(),
            stats: vec![ModStat{stat_type: StatType::AcuityBonus, stat_value: 350}]
        },
        WeaponMod {  // 16
            name: "Primed Cryo Rounds".into_string(),
            stats: vec![ModStat {stat_type: StatType::Cold, stat_value: 165}]
        },
        WeaponMod {  // 17
            name: "Primed Shred".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::FireRate, stat_value: 55},
                ModStat{stat_type: StatType::PunchThrough, stat_value: 12}
            ]
        },
        WeaponMod {  // 18
            name: "Radiated Reload".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Radiation, stat_value: 60},
                ModStat{stat_type: StatType::ReloadSpeed, stat_value: 60}
            ]
        },
        WeaponMod {  // 19
            name: "Rifle Elementalist".into_string(),
            stats: vec![ModStat {stat_type: StatType::StatusDamage, stat_value: 90}]
        },
        WeaponMod {  // 20
            name: "Rime Rounds".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Cold, stat_value: 60},
                ModStat{stat_type: StatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 21
            name: "Semi-Rifle Cannonade".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Damage, stat_value: 0},
                ModStat{stat_type: StatType::SemiAutoCondition, stat_value: 240}
            ]
        },
        WeaponMod {  // 22
            name: "Speed Trigger".into_string(),
            stats: vec![ModStat {stat_type: StatType::FireRate, stat_value: 60}]
        },
        WeaponMod {  // 23
            name: "Stormbringer".into_string(),
            stats: vec![ModStat{stat_type: StatType::Shock, stat_value: 110}]
        },
        WeaponMod {  // 24
            name: "Thermite Rounds".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Heat, stat_value: 60},
                ModStat{stat_type: StatType::StatusChance, stat_value: 60}
            ]
        },
        WeaponMod {  // 25
            name: "Vile Acceleration".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::FireRate, stat_value: 90},
                ModStat{stat_type: StatType::Damage, stat_value: -15}
            ]
        },
        WeaponMod {  // 26
            name: "Vital Sense".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritDamage, stat_value: 120}]
        },
        WeaponMod {  // 27
            name: "Wildfire".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Heat, stat_value: 60},
                ModStat{stat_type: StatType::MagazineCapacity, stat_value: 20}
            ]
        },
        WeaponMod {  // 28
            name: "Vigilante Armaments".into_string(),
            stats: vec![ModStat{stat_type: StatType::Multishot, stat_value: 60}]
        },
        WeaponMod {  // 29
            name: "Vigilante Fervor".into_string(),
            stats: vec![ModStat{stat_type: StatType::FireRate, stat_value: 45}]
        }
    ];
    pub const CANNONADE_INDEX: usize = 21;
    pub const DAMAGE_MODS: [usize; 2] = [0, 8];
    pub const ELE_DAMAGE_MODS: [usize; 4] = [16, 11, 9, 23];  // cold, toxic, heat, shock
    pub const MULTISHOT_MODS: [usize; 2] = [5, 28];
    pub const CRIT_CHANCE_MODS: [usize; 2] = [6, 3];
    pub const CRIT_DAMAGE_MODS: [usize; 3] = [26, 2, 7];
    pub const FIRE_RATE_MODS: [usize; 4] = [25, 17, 22, 29];
}