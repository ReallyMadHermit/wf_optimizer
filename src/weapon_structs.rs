use crate::mod_structs::{WeaponMod, GunStatType};
use crate::supporting_functions::take_input;
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
    base_stat * ((mod_sum + 100) as f32 / 100.0)
}

fn apply_inverse_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    base_stat / ((mod_sum + 100) as f32 / 100.0)
}

#[derive(Clone)]
pub struct GunStats {
    pub fire_rate: f32,
    pub multishot: f32,
    pub magazine: f32,
    pub reload: f32,
    pub semi: bool,
    pub gun_type: GunType,
    pub hit_stats: [HitStats; 2]
} impl GunStats {

    pub fn calculate_shot_damage(&self) -> f32 {
        let mut hit_sum = 0.0;
        for hit in &self.hit_stats {
            hit_sum += hit.damage * (1.0 + (hit.crit_chance * (hit.crit_damage - 1.0)))
        };
        hit_sum *= self.multishot;
        return hit_sum;
    }

    pub fn calculate_burst_dps(&self, shot_damage: f32) -> f32 {
        self.fire_rate * shot_damage
    }

    pub fn calculate_sustained_dps(&self, burst_dps: f32) -> f32 {
        let mag_time = self.magazine / self.fire_rate;
        let firing_ratio = mag_time / (mag_time + self.reload);
        firing_ratio * burst_dps
    }
    
    pub fn generate_report(
        &self, criteria: Criteria, mod_list: &[u8; 8], arcane: u8
    ) -> WeaponReport {
        let hit = self.calculate_shot_damage();
        let burst = self.calculate_burst_dps(hit);
        let sustained = self.calculate_sustained_dps(burst);
        WeaponReport {
            gun_type: self.gun_type.clone(),
            criteria,
            hit_damage: hit.round() as u32,
            burst_dps: burst.round() as u32,
            sustained_dps: sustained.round() as u32,
            mods: mod_list.clone(),
            arcane
        }
    }
    
    pub fn apply_stat_sums(&self, stat_sums: &GunStatModSums) -> Self {
        let mut modded_self = self.clone();
        modded_self.fire_rate = apply_stat_sum(self.fire_rate, stat_sums.fire_rate);
        modded_self.multishot = apply_stat_sum(self.multishot, stat_sums.multishot);
        modded_self.magazine = apply_stat_sum(self.magazine, stat_sums.magazine);
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

#[derive(Clone)]
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
    reload: i16,
    kills: bool,
    semi: bool
} impl GunStatModSums {

    pub fn new(kills: bool, semi: bool) -> Self {
        GunStatModSums {
            damage: 0,
            ele_damage: 0,
            multishot: 0,
            crit_chance: 0,
            crit_damage: 0,
            status: 0,
            fire_rate: 0,
            magazine: 0,
            reload: 0,
            kills,
            semi
        }
    }

    pub fn from_mod_list(weapon_mods: &[u8; 8], loaded_mods: &Vec<WeaponMod>, gun_stats: &GunStats, criteria: &Criteria) -> Self {
        let mut mod_sums = GunStatModSums::new(
            criteria.kills(), gun_stats.semi
        );
        for &mod_id in weapon_mods {
            let weapon_mod: &WeaponMod = &loaded_mods[mod_id as usize];
            mod_sums.add_mod(&weapon_mod, criteria.kills(), gun_stats.semi);
        };
        return mod_sums;
    }

    pub fn add_mod(
        &mut self, weapon_mod: &WeaponMod, kills: bool, semi: bool
    ) {
        for mod_stat in &weapon_mod.mod_stats {
            match mod_stat.stat_type {
                GunStatType::None => {continue;},
                GunStatType::Damage => {
                    self.damage += mod_stat.stat_value;
                },
                GunStatType::DamageForSemiAuto => {
                    if semi {
                        self.damage += mod_stat.stat_value;
                    };
                },
                GunStatType::DamageOnKill => {
                    if kills {
                        self.damage += mod_stat.stat_value;
                    };
                },
                GunStatType::Cold | GunStatType::Toxic |
                GunStatType::Heat | GunStatType::Shock |
                GunStatType::Radiation | GunStatType::Magnetic => {
                    self.ele_damage += mod_stat.stat_value;
                },
                GunStatType::StatusChance => {
                    self.status += mod_stat.stat_value;
                }
                GunStatType::Multishot => {
                    self.multishot += mod_stat.stat_value;
                },
                GunStatType::MultishotOnKill => {
                    if kills {
                        self.multishot += mod_stat.stat_value;
                    };
                },
                GunStatType::CritChance => {
                    self.crit_chance += mod_stat.stat_value;
                },
                GunStatType::CritChanceOnKill => {
                    if kills {
                        self.crit_chance += mod_stat.stat_value;
                    };
                },
                GunStatType::CritDamage => {
                    self.crit_damage += mod_stat.stat_value;
                },
                GunStatType::CritDamageOnKill => {
                    if kills {
                        self.crit_damage += mod_stat.stat_value;
                    };
                },
                GunStatType::FireRate => {
                    self.fire_rate += mod_stat.stat_value;
                },
                GunStatType::MagazineCapacity => {
                    self.magazine += mod_stat.stat_value;
                },
                GunStatType::ReloadSpeed => {
                    self.reload += mod_stat.stat_value;
                },
                _ => {}
            };
        };
    }

    pub fn remove_mod(
        &mut self, weapon_mod: &WeaponMod, kills: bool, semi: bool
    ) {
        for mod_stat in &weapon_mod.mod_stats {
            match mod_stat.stat_type {
                GunStatType::None => {continue},
                GunStatType::Damage => {
                    self.damage -= mod_stat.stat_value;
                },
                GunStatType::DamageForSemiAuto => {
                    if semi {
                        self.damage -= mod_stat.stat_value;
                    };
                },
                GunStatType::DamageOnKill => {
                    if kills {
                        self.damage -= mod_stat.stat_value;
                    };
                },
                GunStatType::Cold | GunStatType::Toxic |
                GunStatType::Heat | GunStatType::Shock |
                GunStatType::Radiation | GunStatType::Magnetic => {
                    self.ele_damage -= mod_stat.stat_value;
                },
                GunStatType::StatusChance => {
                    self.status -= mod_stat.stat_value;
                }
                GunStatType::Multishot => {
                    self.multishot -= mod_stat.stat_value;
                },
                GunStatType::MultishotOnKill => {
                    if kills {
                        self.multishot -= mod_stat.stat_value;
                    };
                },
                GunStatType::CritChance => {
                    self.crit_chance -= mod_stat.stat_value;
                },
                GunStatType::CritChanceOnKill => {
                    if kills {
                        self.crit_chance -= mod_stat.stat_value;
                    };
                },
                GunStatType::CritDamage => {
                    self.crit_damage -= mod_stat.stat_value;
                },
                GunStatType::CritDamageOnKill => {
                    if kills {
                        self.crit_damage -= mod_stat.stat_value;
                    };
                },
                GunStatType::FireRate => {
                    self.fire_rate -= mod_stat.stat_value;
                },
                GunStatType::MagazineCapacity => {
                    self.magazine -= mod_stat.stat_value;
                },
                GunStatType::ReloadSpeed => {
                    self.reload -= mod_stat.stat_value;
                },
                _ => {}
            };
        };
    }

}

#[derive(Clone, Eq, PartialEq)]
pub enum Criteria {
    PerShot,
    BurstDPS,
    SustainedDPS,
    PerShotNoKills,
    BurstDPSNoKills,
    SustainedDPSNoKills
} impl Criteria {

    pub fn kills(&self) -> bool {
        match self {
            Criteria::PerShot | Criteria::BurstDPS | Criteria::SustainedDPS => {
                true
            },
            _ => {
                false
            }
        }
    }

    pub fn determine_criteria() -> Criteria {
        println!();
        println!("Okay, what are we optimizing this for?");
        println!("1: Per-Shot Damage");
        println!("2: Burst DPS");
        println!("3: Sustained DPS");
        println!("4: Per-Shot Damage, without kills");
        println!("5: Burst DPS, without kills");
        println!("6: Sustained DPS, without kills");
        let input = take_input(
            "If you fuck this up, we assume you want 'Per-Shot Damage'"
        );
        return if let Ok(index) = input.parse::<u8>() {
            match index {
                1 => Criteria::PerShot,
                2 => Criteria::BurstDPS,
                3 => Criteria::SustainedDPS,
                4 => Criteria::PerShotNoKills,
                5 => Criteria::BurstDPSNoKills,
                6 => Criteria::SustainedDPSNoKills,
                _ => Criteria::PerShot
            }
        } else {
            Criteria::PerShot
        };
    }

}

pub struct WeaponReport {
    pub gun_type: GunType,
    pub criteria: Criteria,
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
    
    pub fn get_gunstats(&self, gun_type: GunType) -> GunStats {
        GunStats {
            fire_rate: self.get_fire_rate(),
            multishot: self.get_multishot(),
            magazine: self.get_mag_size(),
            reload: self.get_reload(),
            semi: self.get_semi(),
            gun_type,
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