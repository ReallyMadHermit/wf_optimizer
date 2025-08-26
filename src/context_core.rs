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
    pub gun_type: WeaponType,
    pub damage_criteria: DamageCriteria,
    pub kills: bool,
    pub aiming: bool,
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool
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

    pub fn determine_criteria() -> DamageCriteria {
        println!();
        println!("Okay, what are we optimizing this for?");
        println!("1: Per-Shot Damage");
        println!("2: Burst DPS");
        println!("3: Sustained DPS");
        let input = loop_integer_prompt(
            "Please enter the numer corresponding with your preferred criteria.", 1, 3
        );
        return match input {
            1 => DamageCriteria::PerShot,
            2 => DamageCriteria::BurstDPS,
            3 => DamageCriteria::SustainedDPS,
            _ => DamageCriteria::PerShot
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
        let damage = DamageCriteria::determine_criteria();
        let kills = yes_no_prompt("Use kill-reliant benefits", true);
        let aiming = yes_no_prompt("Use aiming-reliant benefits", true);
        let acuity = yes_no_prompt("Use acuity mods", false);
        let amalgam_prompt = match gun_type {
            WeaponType::Rifle | WeaponType::Bow => {
                "Prefer Amalgam Serration"
            },
            WeaponType::Shotgun => {
                "Prefer Amalgam  Shotgun Barrage"
            },
            WeaponType::Pistol => {
                "Prefer Amalgam Diffusion"
            },
            _ => {"YOU SHOULDN'T BE SEEING THIS! BUT DO YOU PREFER AMALGAM MODS!"}
        };
        let prefer_amalgam = yes_no_prompt(amalgam_prompt, true);
        let riven = yes_no_prompt("Use Riven mod", false);
        ModdingContext {
            gun_type,
            damage_criteria: damage,
            kills,
            semi,
            aiming,
            acuity,
            riven,
            prefer_amalgam
        }
    }

}