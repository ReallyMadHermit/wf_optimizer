use crate::mod_structs::{LoadedMods, GunModSums};
use crate::gun_core::GunModdingCriteria;

pub struct GunData {
    pub name: String,
    pub gun_type: GunType,
    pub semi: bool,
    pub gun_stats: GunStats,
} impl GunData {

    pub fn from_csv_line(line: &str) -> Self {  // TODO: add implicit mods here lmao
        let split: Vec<&str> = line.split(",").collect();
        GunData {
            name: String::from(split[1]),
            gun_type: GunType::parse_from_str(split[0]),
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

#[derive(Copy, Clone)]
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

    // pub fn from_imported_gun(imported_gun: &ImportedGun) -> Self {
    //     imported_gun.get_gunstats()
    // }

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
    Rifle,
    Shotgun,
    Pistol,
    Bow
} impl GunType {

    fn parse_from_str(s: &str) -> Self {
        match s {
            "Rifle" => Self::Rifle,
            "Shotgun" => Self::Shotgun,
            "Pistol" => Self::Pistol,
            "Bow" => Self::Bow,
            _ => {
                println!("Weapon type '{}' not found! Using... Rifle!", s);
                Self::Rifle
            }
        }
    }

}

pub struct LiteReport {
    pub criteria_result: u32,
    pub combo_index: u32,
    pub arcane_index: u32  // this is 32 bits because it was free to do so, shrug
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
        loaded_mods: &LoadedMods,
        loaded_arcanes: &LoadedMods
    ) -> String {
        let mut stat_sums = GunModSums::from_mod_list(
            &combinations[self.combo_index as usize],
            loaded_mods
        );
        // stat_sums.add_mod(&loaded_arcanes[self.arcane_index as usize]);
        stat_sums.add_mod(self.arcane_index as u8, loaded_mods);
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
        loaded_mods: &LoadedMods,
        loaded_arcanes: &LoadedMods
    ) -> String {
        let mut names = [""; 8];
        let arcane = &loaded_arcanes.get_mod_name_u8(self.arcane_index as u8);
        for (index, &id) in combinations[self.combo_index as usize].iter().enumerate() {
            names[index] = loaded_mods.get_mod_name_u8(id);
        };
        format!(
            "{}\n{}, {}, {}, {}, {}, {}, {}, {}",
            arcane,
            names[0], names[1], names[2], names[3],
            names[4], names[5], names[6], names[7]
        )
    }

}
