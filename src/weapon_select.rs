use crate::data::GUN_DATA;
use crate::context_core::{WeaponType};
use crate::cli_inputs::UserInput;

pub struct GunData {  // TODO: restructure to include other fire modes
    pub name: &'static str,
    pub fire_mode: &'static str,
    pub gun_type: WeaponType,
    pub semi: bool,
    pub gun_stats: GunStats,
}

#[derive(Clone)]
pub struct GunStats {
    pub fire_rate: f32,
    pub multishot: f32,
    pub magazine: f32,
    pub reload: f32,
    pub hit_stats: [HitStats; 2]  // TODO: update to [Option<HitStats>; 2]
}

#[derive(Copy, Clone)]
pub struct HitStats {
    pub damage: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub status: f32
}

impl GunData {

    pub fn from_csv_line(line: &'static str) -> Self {
        let split: Vec<&'static str> = line.split(",").collect();
        GunData {
            name: split[1],
            fire_mode: split[2],
            gun_type: WeaponType::from_str(split[0]),
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

    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Fire Mode: {}", self.fire_mode);
        println!("Gun Type: {}", self.gun_type.str());
        println!("Semi: {}",self.semi);
        println!("Gun Stats...");
        println!("  Fire Rate: {}", self.gun_stats.fire_rate);
        println!("  Multishot: {}", self.gun_stats.multishot);
        println!("  Magazine: {}", self.gun_stats.magazine);
        println!("  Reload: {}", self.gun_stats.reload);
        println!("  Hit Stats 1:");
        println!("    Damage: {}", self.gun_stats.hit_stats[0].damage);
        println!("    Crit-Chance: {}", self.gun_stats.hit_stats[0].crit_chance);
        println!("    Crit-Damage: {}", self.gun_stats.hit_stats[0].crit_damage);
        println!("    Status: {}", self.gun_stats.hit_stats[0].status);
        if self.gun_stats.hit_stats[1].damage > 0.0 {
            println!("  Hit Stats 2:");
            println!("    Damage: {}", self.gun_stats.hit_stats[1].damage);
            println!("    Crit-Chance: {}", self.gun_stats.hit_stats[1].crit_chance);
            println!("    Crit-Damage: {}", self.gun_stats.hit_stats[1].crit_damage);
            println!("    Status: {}", self.gun_stats.hit_stats[1].status);
        };
    }

    fn parse_bool(s: &str) -> bool {
        s == "TRUE"
    }

}

pub fn weapon_select() -> Option<GunData> {
    let full_csv: Vec<&str> = GUN_DATA.lines().collect();
    let headless_csv = &full_csv[1..];
    loop {
        println!("Enter a weapon's name (it's case sensitive, (out of spite,) of course)");
        println!("Leave blank to enter a weapon's stats manually, for kit guns and incarnon weapons");
        let input = UserInput::new("...Or enter '*' to do them all, lmao (this will take a while)");
        match input {
            Some(UserInput::Full(s)) => {
                return if let Some(index) = weapon_name_search(&s, headless_csv) {
                    Some(GunData::from_csv_line(headless_csv[index]))
                } else {
                    let c = s.chars().next().unwrap();
                    Some(GunData::from_csv_line(headless_csv[weapon_first_letter_search(c, headless_csv)]))
                };
            },
            Some(UserInput::Single(c)) => {
                if c == '*' {
                    return None;
                };
            },
            None => {
                return Some(custom_weapon_input());
            },
            _ => {
                println!("what? try again");
            }
        };
    };

}

fn weapon_name_search(input_string: &str, headless_csv: &[&str]) -> Option<usize> {
    let mut results:Vec<usize> = Vec::with_capacity(6);
    for (index, &line) in headless_csv.iter().enumerate() {
        if input_string == line.split(",").collect::<Vec<&str>>()[1] {
            results.push(index);
        };
    };
    if results.len() > 1 {
       Some(weapon_list_select(Some(results), headless_csv))
    } else if !results.is_empty() {
        Some(results[0])
    } else {
        None
    }
}

fn weapon_first_letter_search(letter: char, headless_csv: &[&str]) -> usize {
    let mut results: Vec<usize> = Vec::with_capacity(36);
    for (index, &line) in headless_csv.iter().enumerate() {
        if letter.eq_ignore_ascii_case(
            &line.split(",").collect::<Vec<&str>>()[1].chars().next().unwrap()
        ) {
            results.push(index)
        };
    };
    weapon_list_select(Some(results), headless_csv)
}

fn weapon_list_select(options: Option<Vec<usize>>, headless_csv: &[&str]) -> usize {
    if let Some(indices) = options {
        let l = indices.len();
        println!("{} results found:", l);
        for (i, &n) in indices.iter().enumerate() {
            let row: Vec<&str> = headless_csv[n].split(",").collect();
            println!("{}. {}; {}", i+1, row[1], row[2]);
        };
        let choice = UserInput::looped_integer_prompt(
            "Please enter a number from above to make a selection.",
            1,
            l,
            1
        );
        indices[choice-1]
    } else {
        let l = headless_csv.len();
        weapon_list_select(Some((0..l).collect::<Vec<usize>>()), headless_csv)
    }
}

fn custom_weapon_input() -> GunData {
    println!("A custom weapon, huh?! Okay well this is gonna be a lot of inputs, read carefully...");
    let semi = UserInput::yes_no_prompt("Is the weapon eligible for Cannonade mods", false);
    let type_integer = UserInput::looped_integer_prompt(
        "Is it a Rifle, Shotgun, or Pistol?\n1. Rifle*\n2. Shotgun\n3. Pistol\n4. Bow",
        1, 3, 1
    );
    let gun_type = match type_integer {
        1 => WeaponType::Rifle,
        2 => WeaponType::Shotgun,
        3 => WeaponType::Pistol,
        4 => WeaponType::Bow,
        _ => WeaponType::Rifle
    };
    let fire_rate = UserInput::f32_loop(
        "What's the weapon's fire rate, in rounds-per-second?"
    );
    let multishot = UserInput::looped_integer_prompt(
        "What's the weapon's base projectile count? (defaults to 1)",
        1, 1000, 1
    ) as f32;
    let magazine = UserInput::looped_integer_prompt(
        "How many rounds are in the weapon's magazine?",
        0, 1000, 0
    ) as f32;
    let reload = UserInput::f32_loop(
        "How long does it take to reload, in seconds?"
    );
    println!("Okay! HitStats time, let's start with the 'impact' damage instance.");
    let hit_stat_1 = {
        let damage = UserInput::looped_integer_prompt(  // TODO: make this use "total damage" per the arsenal, div by MS
            "How much damage does each projectile deal, on hit? (not counting secondary, radial damage)",
            0, 100000, 1
        ) as f32;
        let crit_chance = UserInput::f32_loop(
            "What's the crit chance? Enter it like 0.36 for 36%, 0.5 for 50%, etc"
        );
        let crit_damage = UserInput::f32_loop(
            "What's the crit damage? Enter it like 2.5 for 2.5x, or 3.0 for 3x"
        );
        let status = UserInput::f32_loop(
            "What's the status chance? Enter it the same as crit chance, 0.3 for 30%, 0.45 for 45%, etc"
        );
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    };
    let hit_stat_2 = if UserInput::yes_no_prompt("Is there a second damage instance? Like, a radial after the impact?", false) {
        let damage = UserInput::looped_integer_prompt(
            "How much damage does the secondary instance deal",
            0, 100000, 1
        ) as f32;
        let crit_chance = UserInput::f32_loop(  // TODO: use last hit stat as default?
            "What's the crit chance? Enter it like 0.36 for 36%, 0.5 for 50%, etc"
        );
        let crit_damage = UserInput::f32_loop(  // TODO: use last hit stat as default?
            "What's the crit damage? Enter it like 2.5 for 2.5x, or 3.0 for 3x"
        );
        let status = UserInput::f32_loop(  // TODO: use last hit stat as default?
            "What's the status chance? Enter it the same as crit chance, 0.3 for 30%, 0.45 for 45%, etc"
        );
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    } else {
        HitStats {
            damage: 0.0,
            crit_chance: 0.0,
            crit_damage: 0.0,
            status: 0.0
        }
    };
    let hit_stats = [hit_stat_1, hit_stat_2];
    GunData {
        name: "Custom Gun",
        fire_mode: "Some Fire Mode",
        gun_type,
        semi,
        gun_stats: GunStats {
            fire_rate, multishot, magazine, reload, hit_stats
        }
    }
}
