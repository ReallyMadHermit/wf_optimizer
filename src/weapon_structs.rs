use crate::mod_structs::{WeaponMod, GunStatType};
use crate::gun_core::GunModdingCriteria;
use crate::parsing::ImportedGun;

#[derive(Clone)]
pub struct HitStats {
    pub damage: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub status: f32
} impl HitStats {

    pub const fn new(damage: f32, crit_chance: f32, crit_damage: f32, status: f32) -> Self {
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    }

    pub const fn empty() -> Self {
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
        return modded_self;
    }

}

#[derive(Clone, Eq, PartialEq)]
pub enum GunType {
    Rifle
}

#[derive(Clone)]
pub struct GunModSums {
    damage: i16,
    ele_damage: i16,
    multishot: i16,
    crit_chance: i16,
    crit_damage: i16,
    status: i16,
    fire_rate: i16,
    magazine: i16,
    reload: i16
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

pub struct LiteReport {
    pub criteria_result: u32,
    pub combo_index: u32,
    pub arcane_index: u32
} impl LiteReport {

    pub fn new(
        modded_stats: GunStats,
        damage_criteria: GunModdingCriteria,
        combo_index: usize, arcane_index: usize
    ) -> Self {
        let shot_damage = modded_stats.calculate_shot_damage();
        if damage_criteria == GunModdingCriteria::PerShot {
            return LiteReport {
                criteria_result: u32::MAX - shot_damage as u32,
                combo_index: combo_index as u32,
                arcane_index: arcane_index as u32
            };
        };
        let burst_damage = modded_stats.calculate_burst_dps(shot_damage);
        if damage_criteria == GunModdingCriteria::BurstDPS {
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
        let mut stat_sums = GunModSums::from_mod_list(
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
