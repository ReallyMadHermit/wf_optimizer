use crate::cli_inputs::UserInput;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum WeaponType {
    Rifle,
    Shotgun,
    Pistol,
    Bow,
    All,
    Primary,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DamageCriteria {
    PerShot,
    BurstDPS,
    SustainedDPS
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ModdingContext {  // TODO: add buffs & banes to context
    pub weapon_type: WeaponType,
    pub damage_criteria: DamageCriteria,
    pub kills: bool,
    pub aiming: bool,
    pub headshot: bool,
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool,
    pub debug_numbers: bool,
    pub bane: bool,
    pub prime_bane: bool,
    pub conditions: u8
}

impl WeaponType {

    pub fn from_str(s: &str) -> Self {
        match s {
            "Rifle" => Self::Rifle,
            "Shotgun" => Self::Shotgun,
            "Pistol" => Self::Pistol,
            "Bow" => Self::Bow,
            "All" => Self::All,
            "Primary" => Self::Primary,
            _ => {
                println!("Weapon type '{}' not found! Using... Rifle!", s);
                Self::Rifle
            }
        }
    }

    pub fn is_compatible(gun_type: Self, mod_type: Self) -> bool {
        matches!((gun_type, mod_type),
            (Self::All, _) |
            (Self::Rifle, Self::Rifle | Self::Primary) |
            (Self::Shotgun, Self::Shotgun | Self::Primary) |
            (Self::Pistol, Self::Pistol) |
            (Self::Bow, Self::Bow | Self::Rifle | Self::Primary) |
            (_, Self::All))
    }

    pub fn str(&self) -> &'static str {
        match self {
            Self::Rifle => "Rifle",
            Self::Shotgun => "Shotgun",
            Self::Pistol => "Pistol",
            Self::Bow => "Bow",
            _ => "idk"
        }
    }

}

impl DamageCriteria {

    pub fn criteria_quiz() -> DamageCriteria {
        println!();
        println!("Okay, what are we optimizing this for?");
        println!("1: Per-Shot Damage");
        println!("2: Burst DPS");
        println!("3: Sustained DPS*");
        let input = UserInput::looped_integer_prompt(
            "Please enter the numer corresponding with your preferred criteria.", 1, 3, 3
        );
        match input {
            1 => DamageCriteria::PerShot,
            2 => DamageCriteria::BurstDPS,
            3 => DamageCriteria::SustainedDPS,
            _ => DamageCriteria::SustainedDPS
        }
    }

}

impl ModdingContext {

    pub fn interview_user(gun_type: WeaponType, semi: bool) -> Self {
        let damage = DamageCriteria::criteria_quiz();
        let kills = UserInput::yes_no_prompt("Use kill-reliant benefits", true);
        let aiming = UserInput::yes_no_prompt("Use aiming-reliant benefits", true);
        let headshot = UserInput::yes_no_prompt("Hitting headshots often", false);
        let acuity = if headshot && gun_type != WeaponType::Shotgun {
            UserInput::yes_no_prompt("Use acuity mods", false)
        } else {
            false
        };
        let (amalgam_prompt, default_bool) = match gun_type {
            WeaponType::Rifle | WeaponType::Bow => {
                ("Use Amalgam Serration", true)
            },
            WeaponType::Shotgun => {
                ("Use Amalgam Shotgun Barrage", false)
            },
            WeaponType::Pistol => {
                ("Use Amalgam Diffusion", false)
            },
            WeaponType::All => {
                ("Use amalgam mods", false)
            },
            _ => {("YOU SHOULDN'T BE SEEING THIS! BUT DO YOU WANT TO USE AMALGAM MODS!", true)}
        };
        let prefer_amalgam = UserInput::yes_no_prompt(amalgam_prompt, default_bool);
        let bane = UserInput::yes_no_prompt("Use Bane mods", false);
        let prime_bane = if bane {
            UserInput::yes_no_prompt("Use Prime Bane mods", false)
        } else {
            false
        };
        let debug_numbers = UserInput::yes_no_prompt("show fumky debungk numbgers??", false);
        let conditions = if kills {
            UserInput::looped_integer_prompt("For the sake of Gun-CO, how many statuses does the enemy have? (leave blank for 0)", 0, 14, 0) as u8
        } else {
            0
        };
        let riven = UserInput::yes_no_prompt("Use Riven mod", false);
        ModdingContext {
            weapon_type: gun_type,
            damage_criteria: damage,
            kills,
            semi,
            aiming,
            headshot,
            acuity,
            riven,
            prefer_amalgam,
            debug_numbers,
            bane,
            prime_bane,
            conditions
        }
    }

}