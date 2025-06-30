use crate::cli_inputs::{loop_integer_prompt, yes_no_prompt};
use crate::weapon_structs::GunType;

#[derive(Clone, Eq, PartialEq)]
pub struct GunModdingContext {
    pub gun_type: GunType,
    pub damage: GunModdingCriteria,
    pub kills: bool,
    pub aiming: bool,
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool
} impl GunModdingContext {

    pub fn interview_user(gun_type: GunType, semi: bool) -> Self {
        let damage = GunModdingCriteria::determine_criteria();
        let kills = yes_no_prompt("Use kill-reliant benefits", true);
        let aiming = yes_no_prompt("Use aiming-reliant benefits", true);
        let acuity = yes_no_prompt("Use acuity mods", false);
        let amalgam_prompt = match gun_type {
            GunType::Rifle | GunType::Bow => {
                "Prefer Amalgam Serration"
            },
            GunType::Shotgun => {
                "Prefer Amalgam  Shotgun Barrage"
            },
            GunType::Pistol => {
                "Prefer Amalgam Diffusion"
            },
            _ => {"YOU SHOULDN'T BE SEEING THIS! BUT DO YOU PREFER AMALGAM MODS!"}
        };
        let prefer_amalgam = yes_no_prompt(amalgam_prompt, true);
        let riven = yes_no_prompt("Use Riven mod", false);
        GunModdingContext {
            gun_type,
            damage,
            kills,
            semi,
            aiming,
            acuity,
            riven,
            prefer_amalgam
        }
    }

    // pub fn generate_filters(&self) -> (Vec<u8>, Vec<u8>) {
    //     self.generate_rifle_filters()
    // }

    // fn generate_rifle_filters(&self) -> (Vec<u8>, Vec<u8>) {
    //     let mut required_set: HashSet<u8> = HashSet::with_capacity(10);
    //     let mut disallowed_set: HashSet<u8> = HashSet::with_capacity(10);
    //     required_set.insert(18);
    //     if !self.kills {
    //         disallowed_set.extend([3, 5, 6, 7]);
    //     };
    //     if !self.semi {
    //         disallowed_set.insert(25);
    //     };
    //     if !self.aiming {
    //         disallowed_set.extend(&[2, 3, 7]);
    //     };
    //     if self.acuity {
    //         required_set.insert(17);
    //         disallowed_set.extend(&[6, 28, 31]);
    //     } else {
    //         disallowed_set.insert(17);
    //     };
    //     if self.riven {
    //         required_set.insert(0);
    //     } else {
    //         disallowed_set.insert(0);
    //     };
    //     if self.prefer_amalgam {
    //         required_set.insert(1);
    //         disallowed_set.insert(26);
    //     } else {
    //         disallowed_set.insert(1);
    //     };
    //     required_set.shrink_to_fit();
    //     disallowed_set.shrink_to_fit();
    //     return (required_set.into_iter().collect(), disallowed_set.into_iter().collect());
    // }

}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GunModdingCriteria {
    PerShot,
    BurstDPS,
    SustainedDPS
} impl GunModdingCriteria {

    pub fn determine_criteria() -> GunModdingCriteria {
        println!();
        println!("Okay, what are we optimizing this for?");
        println!("1: Per-Shot Damage");
        println!("2: Burst DPS");
        println!("3: Sustained DPS");
        let input = loop_integer_prompt(
            "Please enter the numer corresponding with your preferred criteria.", 1, 3
        );
        return match input {
            1 => {
                println!("Optimizing for per-shot damage..");
                GunModdingCriteria::PerShot
            },
            2 => {
                println!("Optimizing for burst damage..");
                GunModdingCriteria::BurstDPS
            },
            3 => {
                println!("Optimizing for sustained damage..");
                GunModdingCriteria::SustainedDPS
            },
            _ => {
                println!("Optimizing for per-shot damage...");
                GunModdingCriteria::PerShot
            }
        };
    }

}