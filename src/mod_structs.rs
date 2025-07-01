pub struct LoadedGunMods {
    mod_names: Vec<String>,
    mod_data: Vec<GunModData>,
    included_mods: [u8; 8],
    pub mod_count: u8,
    pub arcane_count: u8
} impl LoadedGunMods {
    
    pub fn new(size: usize) -> Self {
        Self {
            mod_names: Vec::with_capacity(size),
            mod_data: Vec::with_capacity(size),
            included_mods: [0; 8],  // 0 is count, 1 through 7 are mod ids
            mod_count: 0,
            arcane_count: 0
        }
    }
    
    pub fn list_mods(&self) {
        for name in &self.mod_names {
            println!("{}", name);
        };
    }
    
    pub fn len(&self) -> usize {
        self.mod_data.len()
    }
    
    pub fn load_mod(
        &mut self,
        mod_name: &str, 
        stat_type_1: GunStatType,
        stat_value_1: i16,
        stat_type_2: GunStatType,
        stat_value_2: i16,
        arcane: bool
    ) {
        self.mod_names.push(String::from(mod_name));
        self.mod_data.push(GunModData::new(stat_type_1, stat_type_2, stat_value_1, stat_value_2));
        if arcane {
            self.arcane_count += 1;
        } else {
            self.mod_count += 1;
        };
    }

    pub fn get_mod_name_usize(&self, mod_id: usize) -> &str {
        &self.mod_names[mod_id]
    }

    pub fn get_mod_data_usize(&self, mod_id: usize) -> GunModData {
        self.mod_data[mod_id]
    }
    
    pub fn get_mod_name_u8(&self, mod_id: u8) -> &str {
        self.get_mod_name_usize(mod_id as usize)
    }
    
    pub fn get_mod_data_u8(&self, mod_id: u8) -> GunModData {
        self.get_mod_data_usize(mod_id as usize)
    }

    pub fn include_mod(&mut self, mod_id: u8) {
        let i = self.included_mods[0].wrapping_add(1);
        self.included_mods[0] = i;
        self.included_mods[i as usize] = mod_id;
    }

    pub fn included_mods_count(&self) -> u8 {
        self.included_mods[0]
    }

    pub fn included_mods_slice(&self) -> &[u8] {
        let count = self.included_mods[0] as usize;
        &self.included_mods[1..1+count]
    }
    
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GunModData {
    stat_type_1: GunStatType,
    stat_type_2: GunStatType,
    stat_value_1: i16,
    stat_value_2: i16
} impl GunModData {
    
    pub fn new(    stat_type_1: GunStatType,
                   stat_type_2: GunStatType,
                   stat_value_1: i16,
                   stat_value_2: i16
    ) -> Self {
        Self {stat_type_1, stat_type_2, stat_value_1, stat_value_2}
    }
    
    pub fn as_array(&self) -> [(GunStatType, i16); 2] {
        [
            (self.stat_type_1, self.stat_value_1),
            (self.stat_type_2, self.stat_value_2)
        ]
    }
    
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    
    pub fn from_riven_str(s: &str) -> Self {
        match s {
            "C" => Self::Cold,
            "CC" => Self::CritChance,
            "CD" => Self::CritDamage,
            "D" => Self::Damage,
            "E" => Self::Shock,
            "H" => Self::Heat,
            "F" => Self::FireRate,
            "MG" => Self::MagazineCapacity,
            "MS" => Self::Multishot,
            "T" => Self::Toxic,
            "R" => Self::ReloadSpeed,
            "S" => Self::StatusChance,
            _ => Self::None
        }
    }
    
    pub fn to_str(&self) -> &str {
        match self {
            Self::None => "None",
            Self::Damage => "Damage",
            Self::Heat => "Heat",
            Self::Cold => "Cold",
            Self::Toxic => "Toxic",
            Self::Shock => "Shock",
            Self::Magnetic => "Magnetic",
            Self::Radiation => "Radiation",
            Self::Multishot => "Multishot",
            Self::CritChance => "Crit Chance",
            Self::CritDamage => "Crit Damage",
            Self::FireRate => "Firerate",
            Self::StatusChance => "Status Chance",
            Self::ConditionOverload => "Condition Overload",
            Self::MagazineCapacity => "Magazine Capacity",
            Self::ReloadSpeed => "Reload Speed",
            Self::AcuityBonus => "Acuity Bonus",
            Self::StatusDamage => "Status Damage",
            Self::PunchThrough => "Punch Through",
            Self::AmmoEfficiency => "Ammo Efficiency",
            Self::Riven => "Riven"
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
    pub reload: i16,
    pub ammo_efficiency: i16
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
            reload: 100,
            ammo_efficiency: 0
        }
    }

    pub fn from_mod_list(weapon_mods: &[u8], loaded_mods: &LoadedGunMods) -> Self {
        let mut new_sums = GunModSums::new();
        new_sums.add_many_mods(weapon_mods, loaded_mods);
        return new_sums;
    }

    pub fn add_many_mods(&mut self, weapon_mods: &[u8], loaded_mods: &LoadedGunMods) {
        for &mod_id in weapon_mods {
            self.add_mod(mod_id, loaded_mods);
        };
    }

    pub fn add_mod(&mut self, mod_id: u8, loaded_mods: &LoadedGunMods) {
        for (stat_type, stat_value) in loaded_mods.get_mod_data_u8(mod_id).as_array() {
            self.apply_mod(stat_type, stat_value);
        };
    }

    pub fn remove_mod(&mut self, mod_id: u8, loaded_mods: &LoadedGunMods) {
        for (stat_type, stat_value) in loaded_mods.get_mod_data_u8(mod_id).as_array() {
            self.apply_mod(stat_type, -stat_value);
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
