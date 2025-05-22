#[derive(Clone, Eq, PartialEq)]
pub struct WeaponMod {
    pub name: String,
    pub mod_stats: [GunModStat; 2]
}

pub struct RivenMod {
    mod_stat_array: [GunModStat; 4]
}

#[derive(Clone, Eq, PartialEq, Debug)]
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

#[derive(Clone, Eq, PartialEq)]
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

#[derive(Clone)]
pub struct GunModSums {
    pub damage: i16,
    pub ele_damage: i16,
    pub multishot: i16,
    pub crit_chance: i16,
    pub crit_damage: i16,
    pub status: i16,
    pub fire_rate: i16,
    pub magazine: i16,
    pub reload: i16
} impl GunModSums {

    pub fn new() -> Self {
        GunModSums {
            damage: 100,
            ele_damage: 100,
            multishot: 100,
            crit_chance: 100,
            crit_damage: 100,
            status: 100,
            fire_rate: 100,
            magazine: 100,
            reload: 100
        }
    }

    pub fn from_mod_list(weapon_mods: &[u8], loaded_mods: &Vec<WeaponMod>) -> Self {
        let mut new_sums = GunModSums::new();
        new_sums.add_many_mods(weapon_mods, loaded_mods);
        return new_sums;
    }

    pub fn add_many_mods(&mut self, weapon_mods: &[u8], loaded_mods: &Vec<WeaponMod>) {
        for &mod_id in weapon_mods {
            let weapon_mod: &WeaponMod = &loaded_mods[mod_id as usize];
            self.add_mod(&weapon_mod);
        };
    }

    pub fn add_mod(&mut self, weapon_mod: &WeaponMod) {
        for mod_stat in &weapon_mod.mod_stats {
            self.apply_mod(mod_stat.stat_type.clone(), mod_stat.stat_value.clone())
        };
    }

    pub fn remove_mod(&mut self, weapon_mod: &WeaponMod) {
        for mod_stat in &weapon_mod.mod_stats {
            self.apply_mod(mod_stat.stat_type.clone(), -mod_stat.stat_value.clone())
        };
    }

    pub fn apply_mod(&mut self, stat_type: GunStatType, stat_value: i16) {
        match stat_type {
            GunStatType::None => {},
            GunStatType::Damage => {
                self.damage += stat_value;
            },
            GunStatType::Cold | GunStatType::Toxic |
            GunStatType::Heat | GunStatType::Shock |
            GunStatType::Radiation | GunStatType::Magnetic => {
                self.ele_damage += stat_value;
            },
            GunStatType::StatusChance => {
                self.status += stat_value;
            }
            GunStatType::Multishot => {
                self.multishot += stat_value;
            },
            GunStatType::CritChance => {
                self.crit_chance += stat_value;
            },
            GunStatType::CritDamage => {
                self.crit_damage += stat_value;
            },
            GunStatType::FireRate => {
                self.fire_rate += stat_value;
            },
            GunStatType::MagazineCapacity => {
                self.magazine += stat_value;
            },
            GunStatType::ReloadSpeed => {
                self.reload += stat_value;
            },
            _ => {}
        };
    }

}
