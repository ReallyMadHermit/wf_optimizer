use crate::context_core::DamageCriteria;
use crate::mod_parsing::{LoadedMods, ModStatType};
use crate::weapon_select::GunStats;
use crate::combinatorics::BuildCombo;

pub fn calculate_builds(
    combinations: &Vec<BuildCombo>,
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    criteria: DamageCriteria
) -> Vec<SortingHelper> {
    match criteria {
        DamageCriteria::PerShot => calculate_shot_damage(combinations, loaded_mods, base_gun_stats),
        DamageCriteria::BurstDPS => {Vec::new()},  // TODO: complete burst dps criteria path
        DamageCriteria::SustainedDPS => {Vec::new()},  // TODO: complete sustained dps criteria path
    }
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
    combinations: &Vec<BuildCombo>,
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats
) -> Vec<SortingHelper> {
    let mut builds = Vec::with_capacity(combinations.len());
    for (index, build_combo) in combinations.iter().enumerate() {
        let mut mod_sums = GunModSums::from_mod_list(&build_combo.mod_combo, loaded_mods);
        if let Some(a) = build_combo.arcane {
            mod_sums.add_mod(a, loaded_mods);
        };
        builds.push(SortingHelper::new(apply_stat_sums(base_gun_stats, &mod_sums)
                .calculate_shot_damage(), index));
    };
    builds
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

    fn new() -> Self {
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

    fn from_mod_list(weapon_mods: &[u8], loaded_mods: &LoadedMods) -> Self {
        let mut new_sums = GunModSums::new();
        new_sums.add_many_mods(weapon_mods, loaded_mods);
        return new_sums;
    }

    fn add_many_mods(&mut self, weapon_mods: &[u8], loaded_mods: &LoadedMods) {
        for &mod_id in weapon_mods {
            self.add_mod(mod_id, loaded_mods);
        };
    }

    fn add_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        let mod_data = loaded_mods.get_mod(mod_id);
        self.apply_mod(mod_data.stat_type_1, mod_data.stat_value_1);
        self.apply_mod(mod_data.stat_type_2, mod_data.stat_value_2);
    }

    fn remove_mod(&mut self, mod_id: u8, loaded_mods: &LoadedMods) {
        let mod_data = loaded_mods.get_mod(mod_id);
        self.apply_mod(mod_data.stat_type_1, -mod_data.stat_value_1);
        self.apply_mod(mod_data.stat_type_2, -mod_data.stat_value_2);
    }

    fn apply_mod(&mut self, stat_type: ModStatType, stat_value: i16) {
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

fn apply_stat_sums(gun_stats: &GunStats, stat_sums: &GunModSums) -> GunStats {
    let mut modded_self = gun_stats.clone();
    modded_self.fire_rate = apply_stat_sum(gun_stats.fire_rate, stat_sums.fire_rate);
    modded_self.multishot = apply_stat_sum(gun_stats.multishot, stat_sums.multishot);
    modded_self.magazine = apply_stat_sum(gun_stats.magazine, stat_sums.magazine).round();
    modded_self.reload = apply_inverse_stat_sum(gun_stats.reload, stat_sums.reload);
    for i in 0..gun_stats.hit_stats.len() {
        let modded_hit = &mut modded_self.hit_stats[i];
        let base_hit = &gun_stats.hit_stats[i];
        modded_hit.damage = apply_stat_sum(base_hit.damage, stat_sums.damage);
        modded_hit.damage = apply_stat_sum(modded_hit.damage, stat_sums.ele_damage);
        modded_hit.crit_chance = apply_stat_sum(base_hit.crit_chance, stat_sums.crit_chance);
        modded_hit.crit_damage = apply_stat_sum(base_hit.crit_damage, stat_sums.crit_damage);
        modded_hit.status = apply_stat_sum(base_hit.status, stat_sums.status);
    };
    if stat_sums.ammo_efficiency >= 100 {
        modded_self.reload = 0.0;
    } else if stat_sums.ammo_efficiency > 0 {
        modded_self.magazine = apply_ammo_efficiency(modded_self.magazine, stat_sums.ammo_efficiency);
    };
    return modded_self;
}