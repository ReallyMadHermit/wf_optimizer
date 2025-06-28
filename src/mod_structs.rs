pub struct LoadedMods{
    mod_names: Vec<String>,
    stat_types_1: Vec<GunStatType>,
    stat_values_1: Vec<i16>,
    stat_types_2: Vec<GunStatType>,
    stat_values_2: Vec<i16>
} impl LoadedMods {
    
    pub fn new(size: usize) -> Self {
        Self {
            mod_names: Vec::with_capacity(size),
            stat_types_1: Vec::with_capacity(size),
            stat_values_1: Vec::with_capacity(size),
            stat_types_2: Vec::with_capacity(size),
            stat_values_2: Vec::with_capacity(size)
        }
    }
    
    pub fn len(&self) -> usize {
        self.mod_names.len()
    }
    
    pub fn load_mod(
        &mut self,
        mod_name: &str, 
        stat_type_1: GunStatType,
        stat_value_1: i16,
        stat_type_2: GunStatType,
        stat_value_2: i16
    ) {
        self.mod_names.push(String::from(mod_name));
        self.stat_types_1.push(stat_type_1);
        self.stat_values_1.push(stat_value_1);
        self.stat_types_2.push(stat_type_2);
        self.stat_values_2.push(stat_value_2);
    }

    pub fn get_mod_name_usize(&self, mod_id: usize) -> &str {
        &self.mod_names[mod_id]
    }

    pub fn get_mod_data_usize(&self, mod_id: usize) -> [(GunStatType, i16);2] {
        let i = mod_id;
        [
            (self.stat_types_1[i], self.stat_values_1[i]),
            (self.stat_types_2[i], self.stat_values_2[i])
        ]
    }
    
    pub fn get_mod_name_u8(&self, mod_id: u8) -> &str {
        self.get_mod_name_usize(mod_id as usize)
    }
    
    pub fn get_mod_data_u8(&self, mod_id: u8) -> [(GunStatType, i16);2] {
        self.get_mod_data_usize(mod_id as usize)
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

    pub fn from_mod_list(weapon_mods: &[u8], loaded_mods: &LoadedMods) -> Self {
        let mut new_sums = GunModSums::new();
        new_sums.add_many_mods(weapon_mods, loaded_mods);
        return new_sums;
    }

    pub fn add_many_mods(&mut self, weapon_mods: &[u8], loaded_mods: &LoadedMods) {
        for &mod_id in weapon_mods {
            self.add_mod(mod_id, loaded_mods);
        };
    }

    pub fn add_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        for (stat_type, stat_value) in loaded_mods.get_mod_data_u8(mod_id) {
            self.apply_mod(stat_type, stat_value);
        };
    }

    pub fn remove_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        for (stat_type, stat_value) in loaded_mods.get_mod_data_u8(mod_id) {
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
