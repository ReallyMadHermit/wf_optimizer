use crate::data::GUN_DATA;
use crate::context_core::{WeaponType};

pub struct GunData {
    pub name: &'static str,
    pub fire_mode: &'static str,
    pub semi: bool,
    pub gun_stats: GunStats,
}

#[derive(Clone)]
pub struct GunStats {
    pub gun_type: WeaponType,
    pub fire_rate: f32,
    pub multishot: f32,
    pub magazine: f32,
    pub reload: f32,
    pub hit_stats: [HitStats; 2]
}

#[derive(Copy, Clone)]
pub struct HitStats {
    pub damage: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub status: f32
}

impl GunData {

    pub fn from_index(row: usize) -> Self {  // assumes index w/o header
        let rows = Vec::from_iter(GUN_DATA.lines());
        Self::from_csv_line(rows[row+1])
    }

    pub fn from_csv_line(line: &'static str) -> Self {
        let split: Vec<&'static str> = line.split(",").collect();
        GunData {
            name: split[1],
            fire_mode: split[2],
            semi: Self::parse_bool(split[3]),
            gun_stats: GunStats {
                gun_type: WeaponType::from_str(split[0]),
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