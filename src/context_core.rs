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
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool,
    pub bane: bool,
    pub prime_bane: bool,
    pub buffs: bool,
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

    pub fn amalgam(&self) -> &'static str {
        match self {
            Self::Rifle | Self::Bow => "Use Amalgam Serration",
            Self::Shotgun => "Use Amalgam Shotgun Barrage",
            Self::Pistol => "Use Amalgam Barrel Diffusion",
            _ => "Use Amalgam mod"
        }
    }

}

impl DamageCriteria {

    pub(crate) fn default() -> DamageCriteria {
        Self::PerShot
    }

    pub fn str(&self) -> &str {
        match self {
            Self::PerShot => "Per-Shot Damage",
            Self::BurstDPS => "Burst DPS",
            Self::SustainedDPS => "Sustained DPS"
        }
    }

}