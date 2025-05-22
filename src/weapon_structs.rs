use std::collections::HashSet;
use crate::mod_structs::{WeaponMod, GunStatType};
use crate::supporting_functions::{loop_integer_prompt, yes_no_prompt};
use std::fmt::Write;

#[derive(Clone)]
struct HitStats {
    damage: f32,
    crit_chance: f32,
    crit_damage: f32,
    status: f32
} impl HitStats {
    
    const fn new(damage: f32, crit_chance: f32, crit_damage: f32, status: f32) -> Self {
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    }
    
    const fn empty() -> Self {
        HitStats::new(0.0, 0.0, 0.0, 0.0)
    }
    
}

fn apply_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    if mod_sum == 100 {
        return base_stat;
    };
    base_stat * (mod_sum as f32 / 100.0)
}

fn apply_inverse_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    if mod_sum == 100 {
        return base_stat;
    };
    base_stat / (mod_sum as f32 / 100.0)
}

#[derive(Clone)]
pub struct GunStats {
    pub fire_rate: f32,
    pub multishot: f32,
    pub magazine: f32,
    pub reload: f32,
    pub hit_stats: [HitStats; 2]
} impl GunStats {

    pub fn from_imported_gun(imported_gun: &ImportedGun) -> Self {
        imported_gun.get_gunstats()
    }

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
    
    // pub fn generate_report(
    //     &self, criteria: DamageCriteria, mod_list: &[u8; 8], arcane: u8
    // ) -> WeaponReport {
    //     let hit = self.calculate_shot_damage();
    //     let burst = self.calculate_burst_dps(hit);
    //     let sustained = self.calculate_sustained_dps(burst);
    //     WeaponReport {
    //         gun_type: self.gun_type.clone(),
    //         criteria,
    //         hit_damage: hit.round() as u32,
    //         burst_dps: burst.round() as u32,
    //         sustained_dps: sustained.round() as u32,
    //         mods: mod_list.clone(),
    //         arcane
    //     }
    // }
    
    pub fn apply_stat_sums(&self, stat_sums: &GunStatModSums) -> Self {
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
        return modded_self;
    }

}

#[derive(Clone, Eq, PartialEq)]
pub enum GunType {
    Rifle
}

#[derive(Clone)]
pub struct GunStatModSums {
    damage: i16,
    ele_damage: i16,
    multishot: i16,
    crit_chance: i16,
    crit_damage: i16,
    status: i16,
    fire_rate: i16,
    magazine: i16,
    reload: i16
} impl GunStatModSums {

    pub fn new() -> Self {
        GunStatModSums {
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
        let mut new_sums = GunStatModSums::new();
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

#[derive(Clone, Eq, PartialEq)]
pub struct ModdingCriteria {
    pub gun_type: GunType,
    pub damage: DamageCriteria,
    pub kills: bool,
    pub semi: bool,
    pub aiming: bool,
    pub acuity: bool,
    pub riven: bool,
    pub prefer_amalgam: bool
} impl ModdingCriteria {

    pub fn interview_user(gun_type: GunType, semi: bool) -> Self {
        let damage = DamageCriteria::determine_criteria();
        let kills = yes_no_prompt("Use kill-reliant benefits", true);
        let aiming = yes_no_prompt("Use aiming-reliant benefits", true);
        let acuity = yes_no_prompt("Use acuity mods", false);
        let riven = yes_no_prompt("Use Riven mod", false);
        let prefer_amalgam = yes_no_prompt("Prefer Amalgam Serration & Diffusion", true);
        ModdingCriteria {
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

    pub fn generate_filters(&self) -> (Vec<u8>, Vec<u8>) {
        self.generate_rifle_filters()
    }

    fn generate_rifle_filters(&self) -> (Vec<u8>, Vec<u8>) {
        let mut required_set: HashSet<u8> = HashSet::with_capacity(10);
        let mut disallowed_set: HashSet<u8> = HashSet::with_capacity(10);
        required_set.insert(18);
        if !self.kills {
            disallowed_set.extend([3, 5, 6, 7]);
        };
        if !self.semi {
            disallowed_set.insert(25);
        };
        if !self.aiming {
            disallowed_set.extend(&[2, 3, 7]);
        };
        if self.acuity {
            required_set.insert(17);
            disallowed_set.extend(&[6, 28, 31]);
        } else {
            disallowed_set.insert(17);
        };
        if self.riven {
            required_set.insert(0);
        } else {
            disallowed_set.insert(0);
        };
        if self.prefer_amalgam {
            required_set.insert(1);
            disallowed_set.insert(26);
        } else {
            disallowed_set.insert(1);
        };
        required_set.shrink_to_fit();
        disallowed_set.shrink_to_fit();
        return (required_set.into_iter().collect(), disallowed_set.into_iter().collect());
    }

}

#[derive(Clone, Eq, PartialEq)]
pub enum DamageCriteria {
    PerShot,
    BurstDPS,
    SustainedDPS
} impl DamageCriteria {

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

}

pub struct LiteReport {
    pub criteria_result: u32,
    pub combo_index: u32,
    pub arcane_index: u32
} impl LiteReport {

    pub fn new(
        modded_stats: GunStats,
        damage_criteria: DamageCriteria,
        combo_index: usize, arcane_index: usize
    ) -> Self {
        let shot_damage = modded_stats.calculate_shot_damage();
        if damage_criteria == DamageCriteria::PerShot {
            return LiteReport {
                criteria_result: u32::MAX - shot_damage as u32,
                combo_index: combo_index as u32,
                arcane_index: arcane_index as u32
            };
        };
        let burst_damage = modded_stats.calculate_burst_dps(shot_damage);
        if damage_criteria == DamageCriteria::BurstDPS {
            return LiteReport {
                criteria_result: u32::MAX - burst_damage as u32,
                combo_index: combo_index as u32,
                arcane_index: arcane_index as u32
            };
        };
        LiteReport {
            criteria_result: u32::MAX - modded_stats.calculate_sustained_dps(burst_damage) as u32,
            combo_index: combo_index as u32,
            arcane_index: arcane_index as u32
        }
    }

    pub fn get_report_string(
        &self,
        base_gun_stats: &GunStats,
        combinations: &Vec<[u8; 8]>,
        loaded_mods: &Vec<WeaponMod>,
        loaded_arcanes: &Vec<WeaponMod>
    ) -> String {
        let mut stat_sums = GunStatModSums::from_mod_list(
            &combinations[self.combo_index as usize],
            loaded_mods
        );
        stat_sums.add_mod(&loaded_arcanes[self.arcane_index as usize]);
        let modded_stats = base_gun_stats.apply_stat_sums(&stat_sums);
        format!(
            "{}\n{}",
            LiteReport::get_damage_string(&modded_stats),
            self.get_mod_string(
                combinations,
                loaded_mods,
                loaded_arcanes
            )
        )
    }

    fn get_damage_string(modded_gun_stats: &GunStats) -> String {
        let hit = modded_gun_stats.calculate_shot_damage();
        let burst = modded_gun_stats.calculate_burst_dps(hit);
        let sustained = modded_gun_stats.calculate_sustained_dps(burst);
        format!("{}|{}|{}", hit, burst, sustained)
    }

    fn get_mod_string(
        &self,
        combinations: &Vec<[u8; 8]>,
        loaded_mods: &Vec<WeaponMod>,
        loaded_arcanes: &Vec<WeaponMod>
    ) -> String {
        let mut names = [""; 8];
        let arcane = &loaded_arcanes[self.arcane_index as usize].name;
        for (index, &id) in combinations[self.combo_index as usize].iter().enumerate() {
            names[index] = &loaded_mods[id as usize].name;
        };
        format!(
            "{}\n{}, {}, {}, {}, {}, {}, {}, {}",
            arcane,
            names[0], names[1], names[2], names[3],
            names[4], names[5], names[6], names[7]
        )
    }

}

pub struct WeaponReport {
    pub gun_type: GunType,
    pub criteria: DamageCriteria,
    pub hit_damage: u32,
    pub burst_dps: u32,
    pub sustained_dps: u32,
    pub mods: [u8; 8],
    pub arcane: u8
} impl WeaponReport {
    
    pub fn get_report_string(&self, loaded_mods: &Vec<WeaponMod>, loaded_arcanes: &Vec<WeaponMod>) -> String {
        let mut buffer = String::with_capacity(250);
        _ = writeln!(buffer, "Hit|Burst|Sustained").unwrap();
        _ = writeln!(
            buffer,
            "{}|{}|{}", 
            self.hit_damage, 
            self.burst_dps, 
            self.sustained_dps
        ).unwrap();
        _ = writeln!(buffer, "Arcane: {}, Mods:", loaded_arcanes[self.arcane as usize].name).unwrap();
        for i in 0usize..2usize {
            let off = i * 4;
            _ = writeln!(
                buffer, 
                "{}, {}, {}, {}, ",
                loaded_mods[self.mods[off] as usize].name,
                loaded_mods[self.mods[off + 1] as usize].name,
                loaded_mods[self.mods[off + 2] as usize].name,
                loaded_mods[self.mods[off + 3] as usize].name
            ).unwrap();
        };
        buffer.shrink_to_fit();
        buffer
    }
    
}

pub struct ImportedGun<'a> {
    csv_line: &'a str,
    split: Option<Vec<&'a str>>
}impl<'a> ImportedGun<'a> {
    
    pub fn new(csv_line: &'a str) -> Self {
        ImportedGun {
            csv_line,
            split: Some(csv_line.split(",").collect())
        }
    }
    
    pub fn get_gunstats(&self) -> GunStats {
        GunStats {
            fire_rate: self.get_fire_rate(),
            multishot: self.get_multishot(),
            magazine: self.get_mag_size(),
            reload: self.get_reload(),
            hit_stats: self.get_hit_stats()
        }
    }
    
    fn get_field_index_str(&self, index: usize) -> &str {
        self.split.as_ref().unwrap()[index]
    }

    fn get_field_index_f32(&self, index: usize) -> f32 {
        return if let Ok(parsed_value) = self.split.as_ref().unwrap()[index].parse() {
            parsed_value
        } else {
            println!("Failed to load value index {} for {}", index, self.get_name());
            0.0
        };
    }
    
    pub fn get_name(&self) -> &str {
        self.get_field_index_str(0)
    }
    pub fn get_attack(&self) -> &str {
        self.get_field_index_str(1)
    }
    pub fn get_mag_size(&self) -> f32 {
        self.get_field_index_f32(2)
    }
    pub fn get_reload(&self) -> f32 {
        self.get_field_index_f32(3)
    }
    pub fn get_fire_rate(&self) -> f32 {
        self.get_field_index_f32(4)
    }
    pub fn get_multishot(&self) -> f32 {
        self.get_field_index_f32(5)
    }
    pub fn get_semi(&self) -> bool {
        let s = self.get_field_index_str(6);
        if s == "TRUE" {
            true
        } else {
            false
        }
    }
    pub fn get_punch_through(&self) -> f32 {
        self.get_field_index_f32(7)
    }
    pub fn get_hit_stats(&self) -> [HitStats; 2] {
        let damage = self.get_field_index_f32(8);
        let crit_chance = self.get_field_index_f32(9);
        let crit_damage = self.get_field_index_f32(10);
        let status = self.get_field_index_f32(11);
        let hit_stats_1 = HitStats {
            damage,
            crit_chance,
            crit_damage,
            status
        };
        let damage = self.get_field_index_f32(12);
        return if damage < 1.0 {
            [
                hit_stats_1,
                HitStats::empty()
            ]
        } else {
            let crit_chance = self.get_field_index_f32(13);
            let crit_damage = self.get_field_index_f32(14);
            let status = self.get_field_index_f32(15);
            let hit_stats_2 = HitStats {
                damage,
                crit_chance,
                crit_damage,
                status
            };
            [
                hit_stats_1,
                hit_stats_2
            ]
        };
    }
    
}