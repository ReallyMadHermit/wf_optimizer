use crate::context_core::{WeaponType, DamageCriteria, ModdingContext};
use crate::impl_blocks::{apply_ammo_efficiency, apply_inverse_stat_sum, apply_stat_sum};

pub struct GunData {
    pub name: String,
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
    pub hit_stats: [HitStats; 2]
}

#[derive(Copy, Clone)]
pub struct HitStats {
    pub damage: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub status: f32
}

pub fn establish_the_facts() -> (GunData, ModdingContext) {
    let selected_gun = old_weapon_select();
    let mut gun_modding_context = ModdingContext::interview_user(
        selected_gun.gun_type, selected_gun.semi
    );
    return (selected_gun, gun_modding_context);
}

impl GunData {

    pub fn from_csv_line(line: &str) -> Self {
        let split: Vec<&str> = line.split(",").collect();
        GunData {
            name: String::from(split[1]),
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

    fn parse_bool(s: &str) -> bool {
        s == "TRUE"
    }

}

impl GunStats {

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
        if stat_sums.ammo_efficiency >= 100 {
            modded_self.reload = 0.0;
        } else if stat_sums.ammo_efficiency > 0 {
            modded_self.magazine = apply_ammo_efficiency(modded_self.magazine, stat_sums.ammo_efficiency);
        };
        return modded_self;
    }

}

impl HitStats {

    pub const fn new(damage: f32, crit_chance: f32, crit_damage: f32, status: f32) -> Self {
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    }

    pub const fn empty() -> Self {
        HitStats::new(0.0, 0.0, 0.0, 0.0)
    }

}

fn weapon_select() -> GunData {
    let mut results:Vec<Vec<&str>> = Vec::with_capacity(4);
    let full_csv: Vec<&str> = GUN_DATA.lines().collect();
    let headless_csv = &full_csv[1..];
    for &n in headless_csv {
        println!("{}", n);
    };
    println!("Enter the weapon's name (it's case sensitive (out of spite, of course))");
    let input = UserInput::new("Leave blank, or fuck up the input to choose from a list:");
    match input {
        UserInput::Full(s) => {

        },
        UserInput::Single(c)=> {

        },
        _ => {

        }
    }
    for (index, &line) in headless_csv.iter().enumerate() {
        let split: Vec<&str> = line.split(",").collect();
    };
}

pub fn old_weapon_select() -> GunData {
    let mut results:Vec<usize> = Vec::with_capacity(4);
    let full_csv: Vec<&str> = GUN_DATA.lines().collect();
    let csv_lines: &[&str] = &full_csv[1..];
    for n in csv_lines {
        println!("{}", n);
    };
    println!("Enter the weapon's name (it's case sensitive (out of spite, of course))");
    let input = ye_olde_input("Leave blank, or fuck up the input to choose from a list:");
    for (index, &line) in csv_lines.iter().enumerate() {
        if line.split(",").collect::<Vec<_>>()[1] == input.trim() {
            results.push(index)
        };
    };
    let l = results.len();
    if l < 1 {
        return new_weapon_list_select(csv_lines, &input);
    } else if l > 1 {
        return sub_weapon_select(csv_lines, &results);
    };
    GunData::from_csv_line(csv_lines[results[0]])
}

pub fn sub_weapon_select(csv_lines: &[&str], matches: &Vec<usize>) -> GunData {
    let mut buffer = String::with_capacity(300);
    _ = writeln!(buffer, "There are multiple weapons with that name:");
    for (index, &result) in matches.iter().enumerate() {
        let var = csv_lines[result].split(",").collect::<Vec<_>>()[2];
        _ = writeln!(buffer, "{}. {}", index, var);
    };
    _ = writeln!(buffer, "Please choose the variant number you wish to optimize.");
    buffer.shrink_to_fit();
    let input = ye_olde_input(&buffer);
    GunData::from_csv_line(csv_lines[matches[parse_input(&input.trim())]])
}

pub fn new_weapon_list_select(csv_lines: &[&str], last_input: &str) -> GunData {
    let mut buffer = String::with_capacity(16645);  // NOT ARBITRARY (LEN[1&2]+6)
    let empty = last_input == "";
    let input_first = last_input.to_uppercase().chars().next();
    for (index, &line) in csv_lines.iter().enumerate() {
        let split: Vec<&str> = line.split(",").collect();
        let name = split[1];
        let attack = split[2];
        if empty || input_first == name.chars().next() {  // checks if input is empty
            _ = writeln!(buffer, "{}. {} - {}", index, name, attack);
        };
    };
    _ = writeln!(buffer, "Please enter the number corresponding with the weapon you want to customize...");
    buffer.shrink_to_fit();
    let input = ye_olde_input(&buffer);
    GunData::from_csv_line(csv_lines[parse_input(&input.trim())])
}

pub fn loop_integer_prompt(prompt: &str, min: usize, max: usize) -> usize {
    let mut curious = true;
    let mut parsed_int = 0usize;
    while curious {
        let input = ye_olde_input(prompt);
        if let Ok(parsed_input) = input.trim().parse() {
            parsed_int = parsed_input;
        } else {
            println!("That's not a number! Try again...");
            continue;
        };
        if parsed_int > max {
            println!("That number exceeds the index boundary! Try again...")
        } else if parsed_int >= min {
            curious = false;
        } else {
            println!("I had to write an extra condition for people like you! Try again...")
        };
    };
    parsed_int
}

pub fn yes_no_prompt(prompt: &str, prefer_yes: bool) -> bool {
    let ending = if prefer_yes {
        "(Y/n)?"
    } else {
        "(y/N)?"
    };
    let full_prompt = format!("{} {}", prompt, ending);
    let input = ye_olde_input(&full_prompt);
    let lower = input.trim().to_lowercase();
    return if lower == "y" {
        true
    } else if lower == "n" {
        false
    } else {
        prefer_yes
    };
}

pub fn results_prompt(page: usize, results: usize) {
    println!("You are now in viewing mode and may close this at any time, or view other results.");
    println!("For sorting changes, enter a letter: Per-shot Damage (P) Burst DPS (B) Sustained DPS (S)");
    println!("Enter a number to change how many results are shown, or press enter to go to the next page.");
    println!("You are currently on Page {}, viewing {} results per page.", page, results);
}