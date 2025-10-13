use std::time::Instant;
use crate::combinatorics::BuildCombo;
use crate::context_core::{DamageCriteria, ModdingContext, WeaponType};
use crate::mod_parsing::{LoadedMods, ModStatType};
use crate::weapon_select::GunStats;

pub fn get_highest_damage(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    modding_context: &ModdingContext,
    base_sums: Option<GunModSums>,
) -> Option<u32> {
    calculate_builds(
        loaded_mods, base_gun_stats, modding_context, base_sums
    ).first().map(|top| u32::MAX - top.inverse_damage)
}

pub fn calculate_builds(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    modding_context: &ModdingContext,
    base_sums: Option<GunModSums>
) -> Vec<SortingHelper> {
    let sums = if let Some(sums) = base_sums {
        sums
    } else {
        GunModSums::from_conditions(modding_context.conditions)
    };
    if modding_context.debug_numbers {
        print!("Calculating damage...");
    };
    let mut start = Instant::now();
    let mut results = match modding_context.damage_criteria {
        DamageCriteria::PerShot => calculate_shot_damage(loaded_mods, base_gun_stats, sums),
        DamageCriteria::BurstDPS => {calculate_burst_damage(loaded_mods, base_gun_stats, sums)},
        DamageCriteria::SustainedDPS => {calculate_sustained_damage(loaded_mods, base_gun_stats, sums)}
    };
    if modding_context.debug_numbers {
        let d = start.elapsed();
        println!(" Done! {:?}", d);
        print!("Sorting results...");
        start = Instant::now();
    };
    results.sort_by_key(|build| build.inverse_damage);
    if modding_context.debug_numbers {
        let d = start.elapsed();
        println!(" Done! {:?}", d);
    }
    results
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct SortingHelper {
    pub inverse_damage: u32,
    pub index: u32
} impl SortingHelper {

    pub fn new(damage: f32, index: usize) -> Self {
        Self {
            inverse_damage: u32::MAX - damage.round() as u32,
            index: index as u32
        }
    }

    pub fn damage(&self) -> u32 {
        u32::MAX - self.inverse_damage
    }

}

fn calculate_shot_damage(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    base_sums: GunModSums
) -> Vec<SortingHelper> {
    let mut builds = Vec::with_capacity(loaded_mods.combinations.len());
    for (index, build_combo) in loaded_mods.combinations.iter().enumerate() {
        let mut mod_sums = base_sums.clone();
        mod_sums.add_many_mods(&build_combo.mod_combo, loaded_mods);
        if let Some(a) = build_combo.arcane {
            mod_sums.add_mod(a, loaded_mods);
        };
        let modded_stats = apply_mod_sum(base_gun_stats, &mod_sums);
        let shot_damage = modded_stats.shot_damage();
        builds.push(SortingHelper::new(shot_damage, index));
    };
    builds
}

fn calculate_burst_damage(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    base_sums: GunModSums
) -> Vec<SortingHelper> {
    let mut builds = Vec::with_capacity(loaded_mods.combinations.len());
    for (index, build_combo) in loaded_mods.combinations.iter().enumerate() {
        let mut mod_sums = base_sums.clone();
        mod_sums.add_many_mods(&build_combo.mod_combo, loaded_mods);
        if let Some(a) = build_combo.arcane {
            mod_sums.add_mod(a, loaded_mods);
        };
        let modded_stats = apply_mod_sum(base_gun_stats, &mod_sums);
        let shot_damage = modded_stats.shot_damage();
        let burst_damage = modded_stats.burst_damage(shot_damage);
        builds.push(SortingHelper::new(burst_damage, index));
    };
    builds
}

fn calculate_sustained_damage(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    base_sums: GunModSums
) -> Vec<SortingHelper> {
    let mut builds = Vec::with_capacity(loaded_mods.combinations.len());
    for (index, build_combo) in loaded_mods.combinations.iter().enumerate() {
        let mut mod_sums = base_sums.clone();
        mod_sums.add_many_mods(&build_combo.mod_combo, loaded_mods);
        if let Some(a) = build_combo.arcane {
            mod_sums.add_mod(a, loaded_mods);
        };
        let modded_stats = apply_mod_sum(base_gun_stats, &mod_sums);
        let shot_damage = modded_stats.shot_damage();
        let burst_damage = modded_stats.burst_damage(shot_damage);
        let sustained_damage = modded_stats.sustained_dps(burst_damage);
        builds.push(SortingHelper::new(sustained_damage, index));
    };
    builds
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct ModScores {
    pub arcane: Option<i16>,
    pub mod_scores: [(i16, u8); 8]  // (riven score, mod id)
} impl ModScores {
    pub fn new(
        loaded_mods: &LoadedMods,
        base_gun_stats: &GunStats,
        build_combo: BuildCombo,
        damage_criteria: DamageCriteria,
        base_sums: &GunModSums
    ) -> Self {
        let mut full_sums = base_sums.clone();
        full_sums.apply_build_combo(build_combo, loaded_mods);
        let full_damage = calculate_single_build(base_gun_stats, &full_sums, damage_criteria);

        let mut mod_scores = [(0, 0); 8];
        for (i, &mod_id) in build_combo.mod_combo.iter().enumerate() {
            let mut lesser_sums = full_sums.clone();
            lesser_sums.remove_mod(mod_id, loaded_mods);
            let lesser_damage = calculate_single_build(base_gun_stats, &lesser_sums, damage_criteria);
            mod_scores[i] = (i16::MAX - ((full_damage / lesser_damage - 1.0) * 1000.0).round() as i16, mod_id);
        };

        mod_scores.sort_by_key(|&(score, _)| score);
        for i in 0..8usize {
            mod_scores[i].0 = i16::MAX - mod_scores[i].0;
        };

        let arcane = if let Some(a) = build_combo.arcane {
            let mut lesser_sums = full_sums.clone();
            lesser_sums.remove_mod(a, loaded_mods);
            let lesser_damage = calculate_single_build(base_gun_stats, &lesser_sums, damage_criteria);
            Some(((full_damage / lesser_damage - 1.0) * 1000.0).round() as i16)
        } else {
            None
        };

        Self {
            arcane,
            mod_scores
        }

    }
}

fn calculate_single_build(
    base_gun_stats: &GunStats,
    mod_sums: &GunModSums,
    damage_criteria: DamageCriteria
) -> f32 {
    let stats = apply_mod_sum(base_gun_stats, mod_sums);
    let damage = stats.shot_damage();
    if damage_criteria == DamageCriteria::PerShot {
        return damage;
    };
    let burst = stats.burst_damage(damage);
    if damage_criteria == DamageCriteria::BurstDPS {
        return burst;
    };
    stats.sustained_dps(burst)
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
    pub ammo_efficiency: i16,
    pub headshot: f32,
    pub acuity: bool,
    pub cannonade: bool,
    pub conditions: u8,
    pub overload: i16
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
            ammo_efficiency: 0,
            headshot: 1.0,
            acuity: false,
            cannonade: false,
            conditions: 0,
            overload: 0
        }
    }
    
    pub fn from_conditions(conditions: u8) -> Self {
        let mut sums = Self::new();
        sums.conditions = conditions;
        sums
    }

    fn apply_build_combo(&mut self, build_combo: BuildCombo, loaded_mods: &LoadedMods) {
        if let Some(a) = build_combo.arcane {
            self.add_mod(a, loaded_mods);
        };
        self.add_many_mods(&build_combo.mod_combo, loaded_mods);
    }

    fn add_many_mods(&mut self, weapon_mods: &[u8], loaded_mods: &LoadedMods) {
        for &mod_id in weapon_mods {
            self.add_mod(mod_id, loaded_mods);
        };
    }

    fn add_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        let mod_data = loaded_mods.get_data(mod_id);
        for &(stat, value) in mod_data {
            self.apply_mod(stat, value)
        };
    }

    fn remove_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        let mod_data = loaded_mods.get_data(mod_id);
        for &(stat, value) in mod_data {
            self.apply_mod(stat, -value)
        };
    }

    fn apply_mod(&mut self, stat_type: ModStatType, stat_value: i16) {
        match stat_type {
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
            ModStatType::Headshot => {
                let eff = 100 + stat_value.abs();
                let m = eff as f32 / 100.0;
                if stat_value > 0 {
                    self.headshot *= m;
                } else {
                    self.headshot /= m;
                };
            },
            ModStatType::Acuity => {
                // crit chance
                self.crit_chance += stat_value;
                // headshot
                let eff = 100 + stat_value.abs();
                let m = eff as f32 / 100.0;
                self.acuity = stat_value > 0;
                if self.acuity {
                    self.headshot *= m;
                } else {
                    self.headshot /= m;
                };
            },
            ModStatType::Cannonade => {
                self.damage += stat_value;
                self.cannonade = stat_value > 0;
            },
            ModStatType::ConditionOverload => {
                self.overload += stat_value;
            },
            _ => {}
        };
    }

}

impl GunStats {

    pub fn shot_damage(&self) -> f32 {
        let mut hit_sum = 0.0;
        for hit in &self.hit_stats {
            hit_sum += hit.damage * (1.0 + (hit.crit_chance * (hit.crit_damage - 1.0)))
        };
        hit_sum *= self.multishot;
        hit_sum
    }

    pub fn burst_damage(&self, shot_damage: f32) -> f32 {
        if self.magazine != 1.0 || self.gun_type == WeaponType::Bow {
            self.fire_rate * shot_damage
        } else {
            shot_damage / self.reload
        }
    }

    pub fn sustained_dps(&self, burst_dps: f32) -> f32 {
        if self.magazine > 1.0 || self.gun_type == WeaponType::Bow {
            let mag_time = self.magazine / self.fire_rate;
            let firing_ratio = mag_time / (mag_time + self.reload);
            firing_ratio * burst_dps
        } else {
            burst_dps
        }
    }

}

fn apply_bow_fire_rate(base_stat: f32, mod_sum: i16) -> f32 {
    if mod_sum == 100 {
        return base_stat;
    };
    let bow_sum = (mod_sum - 100) * 2 + 100;
    base_stat * (bow_sum as f32 / 100.0)
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

fn apply_ammo_efficiency(mag_size: f32, ammo_efficiency: i16) -> f32 {
    let eff_factor = (100 - ammo_efficiency) as f32 / 100.0;
    mag_size / eff_factor
}

fn apply_mod_sum(gun_stats: &GunStats, stat_sums: &GunModSums) -> GunStats {
    let mut modded_self = gun_stats.clone();
    if gun_stats.gun_type == WeaponType::Bow {
        modded_self.fire_rate = apply_bow_fire_rate(gun_stats.fire_rate, stat_sums.fire_rate);
    } else if !stat_sums.cannonade {
        modded_self.fire_rate = apply_stat_sum(gun_stats.fire_rate, stat_sums.fire_rate);
    };
    if !stat_sums.acuity {
        modded_self.multishot = apply_stat_sum(gun_stats.multishot, stat_sums.multishot);
    };
    modded_self.magazine = apply_stat_sum(gun_stats.magazine, stat_sums.magazine).round();
    modded_self.reload = apply_inverse_stat_sum(gun_stats.reload, stat_sums.reload);
    for i in 0..gun_stats.hit_stats.len() {
        let modded_hit = &mut modded_self.hit_stats[i];
        let damage_sum = if i == 0 {
            modded_hit.damage *= stat_sums.headshot;
            stat_sums.damage + stat_sums.overload * stat_sums.conditions as i16
        } else {
            stat_sums.damage
        };
        modded_hit.damage = apply_stat_sum(modded_hit.damage, damage_sum);
        modded_hit.damage = apply_stat_sum(modded_hit.damage, stat_sums.ele_damage);
        modded_hit.crit_chance = apply_stat_sum(modded_hit.crit_chance, stat_sums.crit_chance);
        modded_hit.crit_damage = apply_stat_sum(modded_hit.crit_damage, stat_sums.crit_damage);
        modded_hit.status = apply_stat_sum(modded_hit.status, stat_sums.status);
    };
    if stat_sums.ammo_efficiency >= 100 {
        modded_self.reload = 0.0;
    } else if stat_sums.ammo_efficiency > 0 {
        modded_self.magazine = apply_ammo_efficiency(modded_self.magazine, stat_sums.ammo_efficiency);
    };
    modded_self
}