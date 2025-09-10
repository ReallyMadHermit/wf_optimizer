use crate::cli_inputs::UserInput;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WeaponType {
    Rifle,
    Shotgun,
    Pistol,
    Bow,
    Riven,
    Primary
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DamageCriteria {
    PerShot,
    BurstDPS,
    SustainedDPS
}

#[derive(Clone, Eq, PartialEq)]
pub struct ModdingContext {
    pub weapon_type: WeaponType,
    pub damage_criteria: DamageCriteria,
    pub kills: bool,
    pub aiming: bool,
    pub headshot: bool,
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool,
    pub timed: bool
}

impl WeaponType {

    pub fn from_str(s: &str) -> Self {
        match s {
            "Rifle" => Self::Rifle,
            "Shotgun" => Self::Shotgun,
            "Pistol" => Self::Pistol,
            "Bow" => Self::Bow,
            "Riven" => Self::Riven,
            "Primary" => Self::Primary,
            _ => {
                println!("Weapon type '{}' not found! Using... Rifle!", s);
                Self::Rifle
            }
        }
    }

    pub fn is_compatible(gun_type: Self, mod_type: Self) -> bool {
        match (gun_type, mod_type) {
            (Self::Rifle, Self::Rifle | Self::Primary) => true,
            (Self::Shotgun, Self::Shotgun | Self::Primary) => true,
            (Self::Pistol, Self::Pistol) => true,
            (Self::Bow, Self::Bow | Self::Rifle | Self::Primary) => true,
            (_, Self::Riven) => true,
            _ => false
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
        return match input {
            1 => DamageCriteria::PerShot,
            2 => DamageCriteria::BurstDPS,
            3 => DamageCriteria::SustainedDPS,
            _ => DamageCriteria::SustainedDPS
        };
    }

    pub fn str(&self) -> &str {
        match self {
            Self::PerShot => "Per-shot damage",
            Self::BurstDPS => "Burst DPS",
            Self::SustainedDPS => "Sustained DPS"
        }
    }

}

impl ModdingContext {

    pub fn interview_user(gun_type: WeaponType, semi: bool) -> Self {
        let damage = DamageCriteria::criteria_quiz();
        let kills = UserInput::yes_no_prompt("Use kill-reliant benefits", true);
        let aiming = UserInput::yes_no_prompt("Use aiming-reliant benefits", true);
        let headshot = UserInput::yes_no_prompt("Hitting headshots often", false);
        let acuity = if headshot {
            UserInput::yes_no_prompt("Use acuity mods", true)
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
            _ => {("YOU SHOULDN'T BE SEEING THIS! BUT DO YOU PREFER AMALGAM MODS!", true)}
        };
        let prefer_amalgam = UserInput::yes_no_prompt(amalgam_prompt, default_bool);
        let riven = UserInput::yes_no_prompt("Use Riven mod", false);
        // let timed = UserInput::yes_no_prompt("Show processing times", false);
        let timed = false;
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
            timed
        }
    }

}