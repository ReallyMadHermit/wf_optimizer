use crate::context_core::{DamageCriteria, ModdingContext, WeaponType};
use crate::weapon_select::GunData;

pub fn load_context(gun_data: &GunData) -> ModdingContext {
    ModdingContext::default()
}

impl Default for ModdingContext {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::All,
            damage_criteria: DamageCriteria::default(),
            kills: true,
            aiming: true,
            headshot: false,
            semi: false,
            acuity: false,
            prefer_amalgam: false,
            riven: false,
            debug_numbers: false,
            bane: false,
            prime_bane: false,
            buffs: false,
            conditions: 0
        }
    }
}

impl Default for WeaponType {
    fn default() -> Self {
        Self::All
    }
}

impl Default for DamageCriteria {
    fn default() -> Self {
        Self::SustainedDPS
    }
}