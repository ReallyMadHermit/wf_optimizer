use crate::mod_structs::{RifleMods, WeaponMod, StatType};

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
    fire_rate: f32,
    multishot: f32,
    magazine: f32,
    reload: f32,
    semi: bool,
    gun_type: GunType,
    hit_stats: [HitStats; 2]
} impl GunStats {

    pub fn calculate_shot_damage(&self) -> f32 {
        let mut hit_sum = 0.0;
        for hit in &self.hit_stats {
            hit_sum += hit.damage * (1.0 + (hit.crit_chance * (hit.crit_damage - 1.0)))
        };
        return hit_sum;
    }

    pub fn calculate_burst_dps(&self, shot_damage: f32) -> f32 {
        let hits_per_second = self.fire_rate * self.multishot;
        shot_damage * hits_per_second
    }

    pub fn calculate_sustained_dps(&self, burst_dps: f32) -> f32 {
        let mag_time = self.magazine / self.fire_rate;
        let firing_ratio = (mag_time + self.reload) / mag_time;
        firing_ratio * burst_dps
    }

    pub fn apply_stat_sums(&self, stat_sums: &GunStatModSums) -> Self {
        let mut modded_self = self.clone();
        modded_self.fire_rate = apply_stat_sum(self.fire_rate, stat_sums.fire_rate);
        modded_self.multishot = apply_stat_sum(self.multishot, stat_sums.multishot);
        modded_self.magazine = apply_stat_sum(self.magazine, stat_sums.magazine);
        modded_self.reload = apply_inverse_stat_sum(self.reload, stat_sums.reload);
        for i in 0..self.hit_stats.len() {
            let mut modded_hit = &mut modded_self.hit_stats[i];
            let self_hit = &self.hit_stats[i];
            modded_hit.damage = apply_stat_sum(self_hit.damage, stat_sums.damage);
            modded_hit.crit_chance = apply_stat_sum(self_hit.damage, stat_sums.damage);
            modded_hit.crit_damage = apply_stat_sum(self_hit.crit_damage, stat_sums.crit_damage);
            modded_hit.status = apply_stat_sum(self_hit.status, stat_sums.status);
        };
        return modded_self;
    }
    
    pub const RIFLE_LIST: [&'static str; 2] = [
        "Prisma Gorgon",
        "Trumna Prime"
    ];

    pub fn gun_lookup(weapon_name: &str) -> Self {
        match weapon_name {
            "Prisma Gorgon" => GunStats::PRISMA_GORGON,
            "Trumna Prime" => GunStats::TRUMNA_PRIME,
            _ => GunStats::EMPTY_GUN
        }
    }
    
    const EMPTY_GUN: GunStats = GunStats {
        fire_rate: 0.0,
        multishot: 0.0,
        magazine: 0.0,
        reload: 0.0,
        semi: false,
        gun_type: GunType::Rifle,
        hit_stats: [
            HitStats::empty(),
            HitStats::empty()
        ]
    };

    const PRISMA_GORGON: GunStats = GunStats {
        fire_rate: 14.7,
        multishot: 1.0,
        magazine: 120.0,
        reload: 3.0,
        semi: false,
        gun_type: GunType::Rifle,
        hit_stats: [
            HitStats {
                damage: 23.0,
                crit_chance: 0.3,
                crit_damage: 2.3,
                status: 0.15
            },
            HitStats::empty()
        ]
    };
    const TRUMNA_PRIME: GunStats = GunStats {
        fire_rate: 4.67,
        multishot: 1.0,
        magazine: 250.0,
        reload: 4.0,
        semi: false,
        gun_type: GunType::Rifle,
        hit_stats: [
            HitStats {
                damage: 85.0,
                crit_chance: 0.24,
                crit_damage: 2.4,
                status: 0.34
            },
            HitStats {
                damage: 50.0,
                crit_chance: 0.24,
                crit_damage: 2.4,
                status: 0.34
            }
        ]
    };

}

#[derive(Clone)]
enum GunType {
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

    pub fn from_mod_list(mod_list: ModList, gun_type: GunType, kills: bool, semi: bool) -> Self {
        let mut mod_sums = GunStatModSums::new(kills, semi);
        let global_mod_list = match gun_type {
            GunType::Rifle => &RifleMods::ALL_MODS
        };
        for mod_id in mod_list.index_array {
            let weapon_mod: &WeaponMod = &global_mod_list[mod_id as usize];
            mod_sums.add_mod(&weapon_mod, kills, semi);
        };
        return mod_sums;
    }

    fn add_mod(
        &mut self, weapon_mod: &WeaponMod, kills: bool, semi: bool
    ) {
        for mod_stat in &weapon_mod.mod_stats {
            match mod_stat.stat_type {
                StatType::None => {continue},
                StatType::Damage => {
                    self.damage += mod_stat.stat_value;
                },
                StatType::DamageForSemiAuto => {
                    if semi {
                        self.damage += mod_stat.stat_value;
                    };
                },
                StatType::DamageOnKill => {
                    if kills {
                        self.damage += mod_stat.stat_value;
                    };
                },
                StatType::Cold | StatType::Toxic |
                StatType::Heat | StatType::Shock |
                StatType::Radiation | StatType::Magnetic => {
                    self.ele_damage += mod_stat.stat_value;
                },
                StatType::StatusChance => {
                    self.status += mod_stat.stat_value;
                }
                StatType::Multishot => {
                    self.multishot += mod_stat.stat_value;
                },
                StatType::MultishotOnKill => {
                    if kills {
                        self.multishot += mod_stat.stat_value;
                    };
                },
                StatType::CritChance => {
                    self.crit_chance += mod_stat.stat_value;
                },
                StatType::CritChanceOnKill => {
                    if kills {
                        self.crit_chance += mod_stat.stat_value;
                    };
                },
                StatType::CritDamage => {
                    self.crit_damage += mod_stat.stat_value;
                },
                StatType::CritDamageOnKill => {
                    if kills {
                        self.crit_damage += mod_stat.stat_value;
                    };
                },
                StatType::FireRate => {
                    self.fire_rate += mod_stat.stat_value;
                },
                StatType::MagazineCapacity => {
                    self.magazine += mod_stat.stat_value;
                },
                StatType::ReloadSpeed => {
                    self.reload += mod_stat.stat_value;
                },
                _ => {}
            };
        };
    }

}

#[derive(Clone)]
pub struct ModList {
    pub index_array: [i8; 8],
    pub arcane_index: i8,
} impl ModList {

    pub fn new() -> Self {
        ModList {
            index_array: [-1; 8],
            arcane_index: -1
        }
    }

}