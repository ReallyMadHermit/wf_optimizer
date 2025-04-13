use crate::mod_structs::{RifleMods, WeaponMod, StatType};

#[derive(Clone)]
struct HitStats {
    damage: f32,
    crit_chance: f32,
    crit_damage: f32,
    status: f32
}

fn apply_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    base_stat * ((mod_sum + 100) as f32 / 100.0)
}

fn apply_inverse_stat_sum(base_stat: f32, mod_sum: i16) -> f32 {
    base_stat / ((mod_sum + 100) as f32 / 100.0)
}

#[derive(Clone)]
struct GunStats {
    fire_rate: f32,
    multishot: f32,
    magazine: f32,
    reload: f32,
    hit_stats: Vec<HitStats>
} impl GunStats {
    fn apply_stat_sums(&self, stat_sum: &GunStatModSums) -> Self {
        let mut modded_self = self.clone();
        modded_self.fire_rate = apply_stat_sum(self.fire_rate, stat_sum.fire_rate);
        modded_self.multishot = apply_stat_sum(self.multishot, stat_sum.multishot);
        modded_self.magazine = apply_stat_sum(self.magazine, stat_sum.magazine);
        modded_self.reload = apply_inverse_stat_sum(self.reload, stat_sum.reload);
        for i in 0..self.hit_stats.len() {
            let mut modded_hit = &mut modded_self.hit_stats[i];
            let self_hit = &self.hit_stats[i];
            modded_hit.damage = apply_stat_sum(self_hit.damage, stat_sum.damage);
            modded_hit.crit_chance = apply_stat_sum(self_hit.damage, stat_sum.damage);
            modded_hit.crit_damage = apply_stat_sum(self_hit.crit_damage, stat_sum.crit_damage);
            modded_hit.status = apply_stat_sum(self_hit.status, stat_sum.status);
        };
        return modded_self;
    }

    fn gun_lookup(weapon_name: &str) -> Self {
        match weapon_name {
            "Prism Gorgon" => GunStats::PRISMA_GORGON,
            _ => {
                println!("Weapon not found! Have a Prisma Gorgon instead!");
                GunStats::PRISMA_GORGON
            }
        }
    }
    const PRISMA_GORGON: GunStats = GunStats {
        fire_rate: 14.7,
        multishot: 1.0,
        magazine: 120.0,
        reload: 3.0,
        hit_stats: vec![
            HitStats {
                damage: 23.0,
                crit_chance: 0.3,
                crit_damage: 2.3,
                status: 0.15
            }
        ]
    };
}
#[derive(Clone)]
enum GunType {
    Rifle
}
#[derive(Clone)]
struct GunStatModSums {
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

    fn new(kills: bool, semi: bool) -> Self {
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

    fn from_mod_list(mod_list: ModList, kills: bool, semi: bool) -> Self {
        let mut mod_sums = GunStatModSums::new(kills, semi);
        let global_mod_list = match mod_list.gun_type {
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
        for mod_stat in weapon_mod.mod_stats {
            match mod_stat.stat_type {
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
struct ModList {
    index_array: [u8; 8],
    gun_type: GunType
} impl ModList {
    fn new(gun_type: GunType) -> Self {
        ModList {
            index_array: [0; 8],
            gun_type
        }
    }
}