use crate::cli_inputs::{loop_integer_prompt, yes_no_prompt};
use crate::structs::*;

pub fn apply_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    if mod_sum == 100 {
        return base_stat;
    };
    base_stat * (mod_sum as f32 / 100.0)
}

pub fn apply_inverse_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    if mod_sum == 100 {
        return base_stat;
    };
    base_stat / (mod_sum as f32 / 100.0)
}

pub fn apply_ammo_efficiency(mag_size: f32, ammo_efficiency: i16) -> f32 {
    let eff_factor = (100 - ammo_efficiency) as f32 / 100.0;
    mag_size / eff_factor
}

impl ModStatType {

    pub fn from_str(string_slice: &str) -> Self {
        return match string_slice {
            "None" => Self::None,
            "Damage" => Self::Damage,
            "Heat" => Self::Heat,
            "Cold" => Self::Cold,
            "Toxic" => Self::Toxic,
            "Shock" => Self::Shock,
            "Magnetic" => Self::Magnetic,
            "Radiation" => Self::Radiation,
            "Multishot" => Self::Multishot,
            "CritChance" => Self::CritChance,
            "CritDamage" => Self::CritDamage,
            "FireRate" => Self::FireRate,
            "StatusChance" => Self::StatusChance,
            "ConditionOverload" => Self::ConditionOverload,
            "MagazineCapacity" => Self::MagazineCapacity,
            "ReloadSpeed" => Self::ReloadSpeed,
            "AcuityBonus" => Self::AcuityBonus,
            "StatusDamage" => Self::StatusDamage,
            "PunchThrough" => Self::PunchThrough,
            "AmmoEfficiency" => Self::AmmoEfficiency,
            "Riven" => Self::Riven,
            _ => {
                println!("{} not found! Using 'None'", string_slice);
                Self::None
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

impl ModData {

    pub fn new(
        stat_type_1: ModStatType,
        stat_type_2: ModStatType,
        stat_value_1: i16,
        stat_value_2: i16
    ) -> Self {
        Self {stat_type_1, stat_type_2, stat_value_1, stat_value_2}
    }

    pub fn as_array(&self) -> [(ModStatType, i16); 2] {
        [
            (self.stat_type_1, self.stat_value_1),
            (self.stat_type_2, self.stat_value_2)
        ]
    }

    pub fn from_split_slice(slice: &[&str]) -> Self {
        let stat_type_1 = ModStatType::from_str(slice[0]);
        let stat_value_1: i16 = if let Ok(parsed) = slice[1].parse() {
            parsed
        } else {
            0
        };
        let stat_type_2 = ModStatType::from_str(slice[2]);
        let stat_value_2: i16 = if let Ok(parsed) = slice[3].parse() {
            parsed
        } else {
            0
        };
        Self {
            stat_type_1,
            stat_type_2,
            stat_value_1,
            stat_value_2
        }
    }

}

impl LoadedMods {

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
        mod_data: ModData,
        arcane: bool
    ) -> u8 {
        self.mod_names.push(String::from(mod_name));
        self.mod_data.push(mod_data);
        if arcane {
            self.arcane_count += 1;
        } else {
            self.mod_count += 1;
        };
        self.len() as u8 - 1
    }

    pub fn get_mod_name_usize(&self, mod_id: usize) -> &str {
        &self.mod_names[mod_id]
    }

    pub fn get_mod_data_usize(&self, mod_id: usize) -> ModData {
        self.mod_data[mod_id]
    }

    pub fn get_mod_name_u8(&self, mod_id: u8) -> &str {
        self.get_mod_name_usize(mod_id as usize)
    }

    pub fn get_mod_data_u8(&self, mod_id: u8) -> ModData {
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

impl GunModSums {

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
        for (stat_type, stat_value) in loaded_mods.get_mod_data_u8(mod_id).as_array() {
            self.apply_mod(stat_type, stat_value);
        };
    }

    pub fn remove_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        for (stat_type, stat_value) in loaded_mods.get_mod_data_u8(mod_id).as_array() {
            self.apply_mod(stat_type, -stat_value);
        };
    }

    pub fn apply_mod(&mut self, stat_type: ModStatType, stat_value: i16) {
        match stat_type {
            ModStatType::None => {},
            ModStatType::Damage => {
                self.damage += stat_value;
            },
            ModStatType::Cold | ModStatType::Toxic |
            ModStatType::Heat | ModStatType::Shock |
            ModStatType::Radiation | ModStatType::Magnetic => {
                self.ele_damage += stat_value;
            },
            ModStatType::StatusChance => {
                self.status += stat_value;
            }
            ModStatType::Multishot => {
                self.multishot += stat_value;
            },
            ModStatType::CritChance => {
                self.crit_chance += stat_value;
            },
            ModStatType::CritDamage => {
                self.crit_damage += stat_value;
            },
            ModStatType::FireRate => {
                self.fire_rate += stat_value;
            },
            ModStatType::MagazineCapacity => {
                self.magazine += stat_value;
            },
            ModStatType::ReloadSpeed => {
                self.reload += stat_value;
            },
            _ => {}
        };
    }

}

impl HitStats {

    pub const fn new(damage: f32, crit_chance: f32, crit_damage: f32, status: f32) -> Self {
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    }

    pub const fn empty() -> Self {
        HitStats::new(0.0, 0.0, 0.0, 0.0)
    }

}

impl GunData {

    pub fn from_csv_line(line: &str) -> Self {
        let split: Vec<&str> = line.split(",").collect();
        GunData {
            name: String::from(split[1]),
            gun_type: WeaponType::from_str(split[0]),
            semi: Self::parse_bool(split[3]),
            gun_stats: GunStats {
                fire_rate: split[7].parse().unwrap(),
                multishot: split[9].parse().unwrap(),
                magazine: split[6].parse().unwrap(),
                reload: split[8].parse().unwrap(),
                hit_stats: [
                    HitStats {
                        damage: split[11].parse().unwrap(),
                        crit_chance: split[12].parse().unwrap(),
                        crit_damage: split[13].parse().unwrap(),
                        status: split[14].parse().unwrap()
                    },
                    HitStats {
                        damage: split[15].parse().unwrap(),
                        crit_chance: split[16].parse().unwrap(),
                        crit_damage: split[17].parse().unwrap(),
                        status: split[18].parse().unwrap()
                    }
                ]
            }
        }
    }

    fn parse_bool(s: &str) -> bool {
        s == "TRUE"
    }

}

impl GunStats {

    pub fn calculate_shot_damage(&self) -> f32 {
        let mut hit_sum = 0.0;
        for hit in &self.hit_stats {
            hit_sum += hit.damage * (1.0 + (hit.crit_chance * (hit.crit_damage - 1.0)))
        };
        hit_sum *= self.multishot;
        return hit_sum;
    }

    pub fn calculate_burst_dps(&self, shot_damage: f32) -> f32 {
        if self.magazine > 1.1 {
            self.fire_rate * shot_damage
        } else {
            shot_damage
        }
    }

    pub fn calculate_sustained_dps(&self, burst_dps: f32) -> f32 {
        if self.magazine > 1.1 {
            let mag_time = self.magazine / self.fire_rate;
            let firing_ratio = mag_time / (mag_time + self.reload);
            firing_ratio * burst_dps
        } else {
            burst_dps / self.reload
        }
    }

    pub fn apply_stat_sums(&self, stat_sums: &GunModSums) -> Self {
        let mut modded_self = self.clone();
        modded_self.fire_rate = apply_stat_sum(self.fire_rate, stat_sums.fire_rate);
        modded_self.multishot = apply_stat_sum(self.multishot, stat_sums.multishot);
        modded_self.magazine = apply_stat_sum(self.magazine, stat_sums.magazine).round();
        modded_self.reload = apply_inverse_stat_sum(self.reload, stat_sums.reload);
        for i in 0..self.hit_stats.len() {
            let modded_hit = &mut modded_self.hit_stats[i];
            let self_hit = &self.hit_stats[i];
            modded_hit.damage = apply_stat_sum(self_hit.damage, stat_sums.damage);
            modded_hit.damage = apply_stat_sum(modded_hit.damage, stat_sums.ele_damage);
            modded_hit.crit_chance = apply_stat_sum(self_hit.crit_chance, stat_sums.crit_chance);
            modded_hit.crit_damage = apply_stat_sum(self_hit.crit_damage, stat_sums.crit_damage);
            modded_hit.status = apply_stat_sum(self_hit.status, stat_sums.status);
        };
        if stat_sums.ammo_efficiency >= 100 {
            modded_self.reload = 0.0;
        } else if stat_sums.ammo_efficiency > 0 {
            modded_self.magazine = apply_ammo_efficiency(modded_self.magazine, stat_sums.ammo_efficiency);
        };
        return modded_self;
    }

}

impl WeaponType {

    pub fn from_str(s: &str) -> Self {
        match s {
            "Rifle" => Self::Rifle,
            "Shotgun" => Self::Shotgun,
            "Pistol" => Self::Pistol,
            "Bow" => Self::Bow,
            "Riven" => Self::Riven,
            "Primary" => Self::Primary,
            _ => {
                println!("Weapon type '{}' not found! Using... Rifle!", s);
                Self::Rifle
            }
        }
    }

    pub fn is_compatible(gun_type: Self, mod_type: Self) -> bool {
        match (gun_type, mod_type) {
            (Self::Rifle, Self::Rifle | Self::Primary) => true,
            (Self::Shotgun, Self::Shotgun | Self::Primary) => true,
            (Self::Pistol, Self::Pistol) => true,
            (Self::Bow, Self::Bow | Self::Rifle | Self::Primary) => true,
            (_, Self::Riven) => true,
            _ => false
        }
    }

}

impl DamageCriteria {

    pub fn determine_criteria() -> DamageCriteria {
        println!();
        println!("Okay, what are we optimizing this for?");
        println!("1: Per-Shot Damage");
        println!("2: Burst DPS");
        println!("3: Sustained DPS");
        let input = loop_integer_prompt(
            "Please enter the numer corresponding with your preferred criteria.", 1, 3
        );
        return match input {
            1 => DamageCriteria::PerShot,
            2 => DamageCriteria::BurstDPS,
            3 => DamageCriteria::SustainedDPS,
            _ => DamageCriteria::PerShot
        };
    }

    pub fn str(&self) -> &str {
        match self {
            Self::PerShot => "Per-shot damage",
            Self::BurstDPS => "Burst DPS",
            Self::SustainedDPS => "Sustained DPS"
        }
    }

}

impl ModBehavior {

    pub fn from_str(mod_behavior_str: &str) -> Self {
        match mod_behavior_str {
            "EXC"  => Self::Exclude,
            "INC"  => Self::Include,
            "PAR"  => Self::Parallel,
            "!EXC" => Self::NotExclude,
            "!INC" => Self::NotExclude,
            "!PAR" => Self::NotParallel,
            _      => Self::NothingSpecial
        }
    }

}

impl GunModdingContext {

    pub fn interview_user(gun_type: WeaponType, semi: bool) -> Self {
        let damage = DamageCriteria::determine_criteria();
        let kills = yes_no_prompt("Use kill-reliant benefits", true);
        let aiming = yes_no_prompt("Use aiming-reliant benefits", true);
        let acuity = yes_no_prompt("Use acuity mods", false);
        let amalgam_prompt = match gun_type {
            WeaponType::Rifle | WeaponType::Bow => {
                "Prefer Amalgam Serration"
            },
            WeaponType::Shotgun => {
                "Prefer Amalgam  Shotgun Barrage"
            },
            WeaponType::Pistol => {
                "Prefer Amalgam Diffusion"
            },
            _ => {"YOU SHOULDN'T BE SEEING THIS! BUT DO YOU PREFER AMALGAM MODS!"}
        };
        let prefer_amalgam = yes_no_prompt(amalgam_prompt, true);
        let riven = yes_no_prompt("Use Riven mod", false);
        GunModdingContext {
            gun_type,
            damage,
            kills,
            semi,
            aiming,
            acuity,
            riven,
            prefer_amalgam
        }
    }

}