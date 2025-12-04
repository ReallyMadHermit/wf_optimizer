use crate::build_calc::GunModSums;
use crate::cli_inputs::UserInput;
use crate::mod_parsing::ModStatType;

const EMPTY_RESPONSE: &str = "Enter your buff effects according to the key, or leave blank to see the key.";
const VALID_RESPONSE: &str = "If these stats are correct, press enter to proceed, or enter new stats now.";

pub fn establish_buff_effects() -> GunModSums {
    let mut buffs: Option<Vec<(ModStatType, i16)>> = None;
    show_buff_key();
    loop {
        let input = if buffs.is_none() {
            UserInput::new(EMPTY_RESPONSE)
        } else {
            UserInput::new(VALID_RESPONSE)
        };
        if let Some(UserInput::Full(s)) = input {
            buffs = parse_buffs(s);
            if let Some(b) = &buffs {
                buff_readback(b);
            } else {
                println!("Couldn't parse that, what?");
            };
        } else if let Some(v) = &buffs {
            return GunModSums::from_buffs(v);
        } else if input.is_none() {
            show_buff_key();
        };
    };
}

fn buff_readback(buffs: &[(ModStatType, i16)]) {
    println!("\nYour buffs are:");
    for &(stat, value) in buffs {
        match stat {
            ModStatType::FinalCritDamage => {
                let e = value as f32 / 100.0;
                println!("+{}x {}", e, stat.to_str());
            },
            _ => {
                println!("+{}% {}", value, stat.to_str())
            }
        };
    };
}

fn parse_buffs(input_string: String) -> Option<Vec<(ModStatType, i16)>> {
    let upper = input_string.to_ascii_uppercase();
    let split: Vec<&str> = upper.split(" ").collect();
    let mut pairs = Vec::with_capacity(split.len() / 2 + 1);
    let mut stat = ModStatType::None;
    let mut value = 0i16;
    let mut value_flag = false;
    let mut stat_flag = false;
    for s in split {
        if let Ok(i) = s.parse() {
            value = i;
            value_flag = true;
        } else if let Ok(f) = s.parse::<f32>() {
            value = (f * 100.0) as i16;
            value_flag = true;
        } else {
            stat = ModStatType::from_buff_key(s);
            stat_flag = true;
        };
        if value_flag & stat_flag {
            pairs.push((stat, value));
            value_flag = false;
            stat_flag = false;
        };
    };
    if !pairs.is_empty() {
        Some(pairs)
    } else {
        None
    }
}

impl GunModSums {
    fn from_buffs(buffs: &[(ModStatType, i16)]) -> Self {
        let mut stat_sums = Self::default();
        for &(stat, value) in buffs {
            stat_sums.apply_mod(stat, value);
        };
        stat_sums
    }
}

impl ModStatType {
    fn from_buff_key(key: &str) -> Self {
        match key {
            "D" => Self::Damage,
            "E" => Self::Elemental,
            "FR" => Self::FireRate,
            "CC" => Self::CritChance,
            "CD" => Self::CritDamage,
            "FCC" => Self::FlatCritChance,
            "FCD" => Self::FinalCritDamage,
            "B" => Self::Bane,
            "RS" => Self::ReloadSpeed,
            "MS" => Self::Multishot,
            "SC" => Self::StatusChance,
            _ => Self::None
        }
    }
}

fn show_buff_key() {
    println!("Use the below stat-keys and a number for each value to describe your buffs");
    println!("D: Damage");
    println!("E: Elemental");
    println!("FR: Fire-Rate");
    println!("-");
    println!("CC: Crit Chance");
    println!("CD: Crit Damage");
    println!("FCC: Flat Crit Chance");
    println!("FCD: Final Crit Damage");
    println!("-");
    println!("B: Bane");
    println!("RS: Reload Speed");
    println!("MS: Multi-Shot");
    println!("SC: Status Chance");
    println!("-");
    println!("Some examples of valid combinations:");
    println!("60 FR 1.3 FCD");
    println!("45 FCC 200 CC");
    println!("D 700 E 300 FR 130");
    println!("As long as keys and values are entered in pairs, they can be in either order, and aren't case-sensitive.");
}