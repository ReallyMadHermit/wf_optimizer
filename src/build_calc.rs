use crate::context_core::{DamageCriteria, ModdingContext, WeaponType};
use crate::mod_parsing::{LoadedMods, ModStatType};
use crate::weapon_select::GunStats;
use crate::tui::build_organization_structs::{BuildShowcase, BucketManager};


pub fn calculate_builds(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    modding_context: &ModdingContext,
    base_sums: Option<GunModSums>
) -> BuildShowcase {
    let base_sums = base_sums.unwrap_or_default();
    let arcanes = loaded_mods.get_arcane_list();

    let mut bucket_manager = BucketManager::new(arcanes.len());
    for (combo_index, mod_combo) in loaded_mods.mod_combinations.iter().enumerate() {
        let mut combo_sums = base_sums;
        combo_sums.add_many_mods(mod_combo, loaded_mods);
        let modded_stats = apply_mod_sum(base_gun_stats, &combo_sums);
        let gun_damage = get_damage(modding_context, &modded_stats, &combo_sums);
        bucket_manager.add(gun_damage, combo_index, 0);
        for (arcane_index, arcane_stats) in arcanes.iter().enumerate() {
            let mut arcane_sums = combo_sums;
            for &(stat_type, value) in arcane_stats.get() {
                arcane_sums.apply_mod(stat_type, value);
            }
            let modded_stats = apply_mod_sum(base_gun_stats, &arcane_sums);
            let gun_damage = get_damage(modding_context, &modded_stats, &arcane_sums);
            bucket_manager.add(gun_damage, combo_index, arcane_index+1);
        }
    }
    BuildShowcase::from_manager(&bucket_manager)
}


fn get_damage(modding_context: &ModdingContext, modded_gun_stats: &GunStats, mod_sums: &GunModSums) -> f32 {
    match modding_context.damage_criteria {
        DamageCriteria::PerShot => {
            modded_gun_stats.shot_damage(mod_sums.empowered, mod_sums.bane)
        },
        DamageCriteria::BurstDPS => {
            modded_gun_stats.burst_damage(
                modded_gun_stats.shot_damage(mod_sums.empowered, mod_sums.bane
                )
            )
        },
        DamageCriteria::SustainedDPS => {
            modded_gun_stats.sustained_dps(
                modded_gun_stats.burst_damage(
                    modded_gun_stats.shot_damage(mod_sums.empowered, mod_sums.bane
                    )
                )
            )
        }
    }
}


fn calculate_single_build(
    base_gun_stats: &GunStats,
    mod_sums: &GunModSums,
    damage_criteria: DamageCriteria
) -> f32 {
    let stats = apply_mod_sum(base_gun_stats, mod_sums);
    let damage = stats.shot_damage(mod_sums.empowered, mod_sums.bane);
    if damage_criteria == DamageCriteria::PerShot {
        return damage;
    };
    let burst = stats.burst_damage(damage);
    if damage_criteria == DamageCriteria::BurstDPS {
        return burst;
    };
    stats.sustained_dps(burst)
}

#[derive(Clone, Copy, Default)]  // TODO: parse innervate
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
    pub overload: i16,
    pub empowered: i16,
    pub bane: i16,
    pub flat_crit_chance: i16,
    pub final_crit_mod: i16
} impl GunModSums {

    pub fn default() -> Self {
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
            overload: 0,
            empowered: 0,
            bane: 100,
            flat_crit_chance: 0,
            final_crit_mod: 0
        }
    }
    
    pub fn from_conditions(conditions: u8) -> Self {
        let mut sums = Self::default();
        sums.conditions = conditions;
        sums
    }

    fn apply_build_combo(&mut self, build_combo: &[u8], loaded_mods: &LoadedMods) {
        // if let Some(a) = build_combo.arcane {
        //     self.add_mod(a, loaded_mods);
        // };
        self.add_many_mods(&build_combo, loaded_mods);
    }

    fn add_many_mods(&mut self, weapon_mods: &[u8], loaded_mods: &LoadedMods) {
        for &mod_id in weapon_mods {
            self.add_mod_id(mod_id, loaded_mods);
        };
    }

    fn add_mod_id(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
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

    pub fn apply_mod(&mut self, stat_type: ModStatType, stat_value: i16) {
        match stat_type {
            ModStatType::Damage => {
                self.damage += stat_value;
            },
            ModStatType::Cold | ModStatType::Toxic |
            ModStatType::Heat | ModStatType::Shock |
            ModStatType::Radiation | ModStatType::Magnetic
            | ModStatType::Elemental => {
                self.ele_damage += stat_value;
            }
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
            ModStatType::FlatCritChance => {
                self.flat_crit_chance += stat_value;
            },
            ModStatType::FinalCritDamage => {
                self.final_crit_mod += stat_value;
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
            ModStatType::Headshot => {  // TODO: this is how Secondary Surge applies, resolve that
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
            ModStatType::Empowered => {
                self.empowered += stat_value;
            },
            ModStatType::Bane => {
                self.bane += stat_value;
            },
            _ => {}
        };
    }

}

impl GunStats {

    pub fn shot_damage(&self, empowered: i16, bane: i16) -> f32 {
        let mut hit_sum = 0.0;
        for hit in &self.hit_stats {
            if hit.damage <= 0.0 {
                continue;
            };
            hit_sum += hit.damage * (1.0 + (hit.crit_chance * (hit.crit_damage - 1.0)));
            if empowered > 0 {
                let bane_eff = apply_stat_sum(1.0, bane);
                hit_sum += hit.status * (bane_eff * empowered as f32);
            };
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
        modded_hit.crit_chance += stat_sums.flat_crit_chance as f32 / 100.0;
        modded_hit.crit_damage = apply_stat_sum(modded_hit.crit_damage, stat_sums.crit_damage);
        modded_hit.crit_damage += stat_sums.final_crit_mod as f32 / 100.0;
        modded_hit.status = apply_stat_sum(modded_hit.status, stat_sums.status);
    };
    if stat_sums.ammo_efficiency >= 100 {
        modded_self.reload = 0.0;
    } else if stat_sums.ammo_efficiency > 0 {
        modded_self.magazine = apply_ammo_efficiency(modded_self.magazine, stat_sums.ammo_efficiency);
    };
    modded_self
}