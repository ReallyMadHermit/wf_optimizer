#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
struct WeaponMod {
    name: String,
    stats: Vec<ModStat>
}
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
enum StatType {
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
    OnKillBonus,  // added to previous stat if conditions allow
    MagazineCapacity,
    ReloadSpeed,
    AcuityBonus,  // crit chance + weak point damage
    StatusDamage,
    PunchThrough,
    SemiAutoCondition
}
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
struct ModStat {
    stat_type: StatType,
    stat_value: i16
}

struct RifleMods;
impl RifleMods {
    const MOD_COUNT: usize = 28;
    const ALL_MODS: [WeaponMod; RifleMods::MOD_COUNT] = [
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
                ModStat{stat_type: StatType::OnKillBonus, stat_value: 120}
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
                ModStat{stat_type: StatType::OnKillBonus, stat_value: 150}
            ]
        },
        WeaponMod {  // 6
            name: "Galvanized Scope".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::CritChance, stat_value: 120},
                ModStat{stat_type: StatType::OnKillBonus, stat_value: 200}
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
        }
    ];
    const DAMAGE_MODS: [WeaponMod; 2] = [
        WeaponMod {
            name: "Serration".into_string(),
            stats: vec![ModStat {stat_type: StatType::Damage, stat_value: 155}]
        },
        WeaponMod {
            name: "Heavy Caliber".into_string(),
            stats: vec![ModStat {stat_type: StatType::Damage, stat_value: 165}]
        }
    ];
    const ELE_DAMAGE_MODS: [WeaponMod; 4] = [
        WeaponMod {
            name: "Primed Cryo Rounds".into_string(),
            stats: vec![ModStat {stat_type: StatType::Cold, stat_value: 110}]
        },
        WeaponMod {
            name: "Flat Ele Damage 1".into_string(),
            stats: vec![ModStat {stat_type: StatType::Heat, stat_value: 110}]
        },
        WeaponMod {
            name: "Flat Ele Damage 2".into_string(),
            stats: vec![ModStat {stat_type: StatType::Toxic, stat_value: 110}]
        },
        WeaponMod {
            name: "Flat Ele Damage 3".into_string(),
            stats: vec![ModStat {stat_type: StatType::Heat, stat_value: 110}]
        }
    ];
    const MULTISHOT_MODS: [WeaponMod; 2] = [
        WeaponMod {
            name: "Galvanized Chamber".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::Multishot, stat_value: 80},
                ModStat{stat_type: StatType::OnKillBonus, stat_value: 150}
            ]
        },
        WeaponMod {
            name: "Vigilante Armaments".into_string(),
            stats: vec![ModStat{stat_type: StatType::Multishot, stat_value: 60}]
        }
    ];
    const CRIT_CHANCE_MODS: [WeaponMod; 2] = [
        WeaponMod {
            name: "Critical Delay".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::CritChance, stat_value: 200},
                ModStat{stat_type: StatType::FireRate, stat_value: -20}
            ]
        },
        WeaponMod {
            name: "Point Strike".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritChance, stat_value: 150}]
        }
    ];
    const CRIT_DAMAGE_MODS: [WeaponMod; 3] = [
        WeaponMod {
            name: "Vital Sense".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritDamage, stat_value: 120}]
        },
        WeaponMod {
            name: "Bladed Rounds".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritDamage, stat_value: 120}]
        },
        WeaponMod {
            name: "Hammer Shot".into_string(),
            stats: vec![ModStat{stat_type: StatType::CritDamage, stat_value: 60}]
        }
    ];
    const FIRE_RATE_MODS: [WeaponMod; 4] = [
        WeaponMod {
            name: "Vile Acceleration".into_string(),
            stats: vec![
                ModStat{stat_type: StatType::FireRate, stat_value: 90},
                ModStat{stat_type: StatType::Damage, stat_value: -15}
            ]
        },
        WeaponMod {
            name: "Speed Trigger".into_string(),
            stats: vec![ModStat{stat_type: StatType::FireRate, stat_value: 60}]
        },
        WeaponMod {
            name: "Primed Shred".into_string(),
            stats: vec![ModStat{stat_type: StatType::FireRate, stat_value: 55}]
        },
        WeaponMod {
            name: "Vigilante Fervor".into_string(),
            stats: vec![ModStat{stat_type: StatType::FireRate, stat_value: 45}]
        }
    ];
}