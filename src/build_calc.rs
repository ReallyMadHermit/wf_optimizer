use crate::context_core::{DamageCriteria, ModdingContext};
use crate::mod_parsing::{LoadedMods, ModStatType, RivenMod};
use crate::weapon_select::GunStats;

// TODO: write simple function to find single, rop-rated build

pub fn calculate_builds(
    loaded_mods: &LoadedMods,
    base_gun_stats: &GunStats,
    modding_context: &ModdingContext,
    base_sums: Option<GunModSums>
) -> Vec<SortingHelper> {
    let sums = if let Some(sums) = base_sums {
        sums
    } else {
        GunModSums::new()
    };
    let mut results = match modding_context.damage_criteria {
        DamageCriteria::PerShot => calculate_shot_damage(loaded_mods, base_gun_stats, sums),
        DamageCriteria::BurstDPS => {calculate_burst_damage(loaded_mods, base_gun_stats, sums)},
        DamageCriteria::SustainedDPS => {calculate_sustained_damage(loaded_mods, base_gun_stats, sums)}
    };
    results.sort_by_key(|build| build.inverse_damage);
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
        let modded_stats = apply_stat_sums(base_gun_stats, &mod_sums);
        let shot_damage = modded_stats.calculate_shot_damage();
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
        let modded_stats = apply_stat_sums(base_gun_stats, &mod_sums);
        let shot_damage = modded_stats.calculate_shot_damage();
        let burst_damage = modded_stats.calculate_burst_dps(shot_damage);
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
        let modded_stats = apply_stat_sums(base_gun_stats, &mod_sums);
        let shot_damage = modded_stats.calculate_shot_damage();
        let burst_damage = modded_stats.calculate_burst_dps(shot_damage);
        let sustained_damage = modded_stats.calculate_sustained_dps(burst_damage);
        builds.push(SortingHelper::new(sustained_damage, index));
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
    pub ammo_efficiency: i16,
    pub headshot: f32
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
        }
    }

    pub fn from_mod_list(weapon_mods: &[u8], loaded_mods: &LoadedMods) -> Self {
        let mut new_sums = GunModSums::new();
        new_sums.add_many_mods(weapon_mods, loaded_mods);
        return new_sums;
    }

    pub fn from_riven(riven_mod: &RivenMod) -> Self {
        let mut new_sums = Self::new();
        for &(stat_type, stat_value) in &riven_mod.stats {
            new_sums.apply_mod(stat_type, stat_value);
        };
        new_sums
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
                if stat_value > 0 {
                    let eff = 100 + stat_value;
                    let m = eff as f32 / 100.0;
                    self.headshot *= m;
                };
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
        if i == 0 {
            modded_hit.damage *= stat_sums.headshot;
        };
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