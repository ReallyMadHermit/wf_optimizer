use std::io::stdin;

pub enum UserInput {
    Full(String),
    Single(char),
    Digit(usize)
} impl UserInput {

    pub fn new(prompt: &str) -> Option<Self> {
        let input = Self::cli_input(prompt);
        if let Some(integer) = Self::parse_integer(&input) {
            return Some(Self::Digit(integer));
        } else if let Some(s) = input {
            return if s.len() > 1 {
                Some(Self::Full(s))
            } else { s.chars().nth(0).map(Self::Single) };
        };
        None
    }

    pub fn yes_no_prompt(prompt: &str, prefer_yes: bool) -> bool {
        let ending = if prefer_yes {
            "(Y/n)?"
        } else {
            "(y/N)?"
        };
        let full_prompt = format!("{} {}", prompt, ending);
        let input = UserInput::new(&full_prompt);
        if let Some(UserInput::Single(c)) = input {
            let cl = c.to_ascii_lowercase();
            if cl == 'y' {
                return true;
            } else if cl == 'n' {
                return false;
            };
        };
        prefer_yes
    }
    
    pub fn looped_integer_prompt(prompt: &str, min: usize, max: usize, default_value: usize) -> usize {
        loop {
            let response = if let Some(ui) = UserInput::new(prompt) {
                ui
            } else {
                return default_value;
            };
            match response {
                UserInput::Digit(d) => {
                    if d >= min && d <= max {
                        return d;
                    } else {
                        println!("That number exceeds the index boundary! Try again...")
                    };
                },
                _ => {
                    println!("That's not a number! Try again...");
                }
            };
        };
    }

    pub fn looped_integer_prompt_simple(prompt: &str) -> usize {
        loop {
            let response = if let Some(ui) = UserInput::new(prompt) {
                ui
            } else {
                continue;
            };
            match response {
                UserInput::Digit(d) => {
                    return d;
                },
                _ => {
                    println!("That's not a number! Try again...");
                }
            };
        };
    }

    pub fn f32_loop(prompt: &str, default: Option<f32>) -> f32 {
        loop {
            let r = Self::f32(prompt);
            if let Some(f) = r {
                return f;
            } else if let Some (d) = default {
                return d;
            };
        }
    }

    pub fn f32(prompt: &str) -> Option<f32> {
        let i = Self::new(prompt);
        match i {
            Some(UserInput::Full(s)) => {
                let r = s.parse::<f32>();
                r.ok()
            },
            Some(UserInput::Digit(e)) => {
                Some(e as f32)
            },
            _ => None
        }
    }

    fn cli_input(prompt: &str) -> Option<String> {
        let mut buffer = String::with_capacity(25);
        println!("{}", prompt);
        let _ = stdin().read_line(&mut buffer);
        buffer = String::from(buffer.trim());
        if !buffer.is_empty() {
            buffer.shrink_to_fit();
            Some(buffer)
        } else {
            None
        }
    }

    fn parse_integer(input: &Option<String>) -> Option<usize> {
        input.as_ref()?.parse().ok()
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
        "What's the weapon's fire rate, in rounds-per-second?",
        None
    );
    let multishot = UserInput::looped_integer_prompt(
        "What's the weapon's base projectile count? (defaults to 1)",
        1, 1000, 1
    );
    let magazine = UserInput::looped_integer_prompt_simple(
        "How many rounds are in the weapon's magazine?",
    ) as f32;
    let reload = UserInput::f32_loop(
        "How long does it take to reload, in seconds?",
        Some(0.0)
    );
    println!("Okay! HitStats time, let's start with the 'impact' damage instance.");
    let hit_stat_1 = {
        let damage = UserInput::looped_integer_prompt_simple(
            "How much damage does each projectile deal, on impact?",
        ) as f32;
        let crit_chance = UserInput::f32_loop(
            "What's the crit chance? Enter it as 36 for 36%, 50 for 50%, etc",
            None
        ) / PERCENT_DIV;
        let crit_damage = UserInput::f32_loop(
            "What's the crit damage? Enter it like 2.5 for 2.5x, or 3.0 for 3x",
            None
        );
        let status = UserInput::f32_loop(
            "What's the status chance? Enter it the same as crit chance, 30 for 30%, 45 for 45%, etc",
            None
        ) / PERCENT_DIV;
        HitStats {
            damage, crit_chance, crit_damage, status
        }
    };
    let hit_stat_2 = if UserInput::yes_no_prompt("Is there a second damage instance? Like, a radial after the impact?", false) {
        let damage = UserInput::looped_integer_prompt_simple(
            "How much damage per-projectile?",
        ) as f32;
        let crit_chance = UserInput::f32_loop(
            "What's the crit chance? Press enter to use the same crit chance as above.",
            Some(hit_stat_1.crit_chance * 100.0)
        ) / PERCENT_DIV;
        let crit_damage = UserInput::f32_loop(
            "What's the crit damage? Press enter to use the same crit damage as above.",
            Some(hit_stat_1.crit_damage)
        );
        let status = UserInput::f32_loop(
            "What's the status chance? Press enter to use the same status chance as above.",
            Some(hit_stat_1.status * 100.0)
        ) / PERCENT_DIV;
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
        semi,
        gun_stats: GunStats {
            gun_type, fire_rate, multishot: multishot as f32, magazine, reload, hit_stats
        }
    }
}


const PERCENT_DIV: f32 = 100.0;


impl GunData {

    pub fn print(&self) {
        println!("Name: {}", self.name);
        println!("Fire Mode: {}", self.fire_mode);
        println!("Gun Type: {}", self.gun_stats.gun_type.str());
        println!("Semi: {}",self.semi);
        println!("Gun Stats...");
        println!("  Fire Rate: {}/s", self.gun_stats.fire_rate);
        println!("  Multishot: {}", self.gun_stats.multishot);
        println!("  Magazine: {}", self.gun_stats.magazine);
        println!("  Reload: {}s", self.gun_stats.reload);
        println!("  Hit Stats 1:");
        println!("    Damage: {}", self.gun_stats.hit_stats[0].damage);
        println!("    Crit-Chance: {}%", self.gun_stats.hit_stats[0].crit_chance * PERCENT_DIV);
        println!("    Crit-Damage: {}", self.gun_stats.hit_stats[0].crit_damage);
        println!("    Status-Chance: {}%", self.gun_stats.hit_stats[0].status * PERCENT_DIV);
        if self.gun_stats.hit_stats[1].damage > 0.0 {
            println!("  Hit Stats 2:");
            println!("    Damage: {}", self.gun_stats.hit_stats[1].damage);
            println!("    Crit-Chance: {}%", self.gun_stats.hit_stats[1].crit_chance * PERCENT_DIV);
            println!("    Crit-Damage: {}", self.gun_stats.hit_stats[1].crit_damage);
            println!("    Status-Chance: {}%", self.gun_stats.hit_stats[1].status * PERCENT_DIV);
        };
    }

}


impl ModData {

    pub fn from_riven_str(input: &str) -> Option<Self> {
        let upper = input.to_ascii_uppercase();
        let mut mod_data = Self::empty();
        let mut stat_type = ModStatType::None;
        let mut stat_value = 0i16;
        let mut value_flag = false;
        let mut type_flag = false;
        for s in upper.split(" ") {
            if let Ok(i) = s.parse() {
                stat_value = i;
                value_flag = true;
            } else {
                stat_type = ModStatType::from_riven_str(s);
                type_flag = true;
            };
            if value_flag & type_flag {
                mod_data.push(stat_type, stat_value);
                value_flag = false;
                type_flag = false;
            };
        };
        if mod_data.count > 0 {
            Some(mod_data)
        } else {
            None
        }
    }

}