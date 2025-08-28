use crate::data::GUN_DATA;
use crate::context_core::{WeaponType, ModdingContext};
use crate::cli_inputs::UserInput;

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

pub fn weapon_select() -> GunData {
    let full_csv: Vec<&str> = GUN_DATA.lines().collect();
    let headless_csv = &full_csv[1..];
    println!("Enter the weapon's name (it's case sensitive, (out of spite,) of course)");
    let input = UserInput::new("Leave blank, or fuck up the input to choose from a list:");
    return match input {
        Some(UserInput::Full(s)) => {
            if let Some(index) = weapon_name_search(&s, headless_csv) {
                GunData::from_csv_line(headless_csv[index])
            } else {
                let c = s.chars().next().unwrap();
                GunData::from_csv_line(headless_csv[weapon_first_letter_search(c, headless_csv)])
            }
        },
        Some(UserInput::Single(c)) => {
            GunData::from_csv_line(headless_csv[weapon_first_letter_search(c, headless_csv)])
        },
        _ => {
            GunData::from_csv_line(headless_csv[weapon_list_select(None, headless_csv)])
        }
    }
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
    } else if results.len() > 0 {
        Some(results[0])
    } else {
        None
    }
}

fn weapon_first_letter_search(letter: char, headless_csv: &[&str]) -> usize {
    let mut results: Vec<usize> = Vec::with_capacity(36);
    for (index, &line) in headless_csv.iter().enumerate() {
        if letter == line.split(",").collect::<Vec<&str>>()[1].chars().next().unwrap() {
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

fn results_prompt(page: usize, results: usize) { // TODO: remove this or reuse it elsehwere
    println!("You are now in viewing mode and may close this at any time, or view other results.");
    println!("For sorting changes, enter a letter: Per-shot Damage (P) Burst DPS (B) Sustained DPS (S)");
    println!("Enter a number to change how many results are shown, or press enter to go to the next page.");
    println!("You are currently on Page {}, viewing {} results per page.", page, results);
}