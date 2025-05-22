#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct WeaponMod {
    pub name: String,
    pub mod_stats: [GunModStat; 2]
}

pub struct RivenMod {
    mod_stat_array: [GunModStat; 4]
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum GunStatType {
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
    MagazineCapacity,
    ReloadSpeed,
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
            "Heat" => GunStatType::Heat,
            "Cold" => GunStatType::Cold,
            "Toxic" => GunStatType::Toxic,
            "Shock" => GunStatType::Shock,
            "Magnetic" => GunStatType::Magnetic,
            "Radiation" => GunStatType::Radiation,
            "Multishot" => GunStatType::Multishot,
            "CritChance" => GunStatType::CritChance,
            "CritDamage" => GunStatType::CritDamage,
            "FireRate" => GunStatType::FireRate,
            "StatusChance" => GunStatType::StatusChance,
            "ConditionOverload" => GunStatType::ConditionOverload,
            "MagazineCapacity" => GunStatType::MagazineCapacity,
            "ReloadSpeed" => GunStatType::ReloadSpeed,
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
pub struct GunModStat {
    pub stat_type: GunStatType,
    pub stat_value: i16
} impl GunModStat {
    
    const fn new(stat_type: GunStatType, stat_value: i16) -> Self {
        GunModStat {
            stat_type,
            stat_value
        }
    }
    
    const fn empty() -> Self {
        GunModStat {
            stat_type: GunStatType::None,
            stat_value: 0
        }
    }
    
}
