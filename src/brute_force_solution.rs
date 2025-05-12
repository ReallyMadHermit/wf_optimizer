use rayon;

use crate::mod_structs::WeaponMod;
use crate::weapon_structs::{Criteria, GunStats, WeaponReport, GunStatModSums};
use std::cmp::Reverse;

// Vec<[u8;8]>
pub fn generate_combinations(index_count: u8) -> Vec<[u8;8]>  {
    let combination_count = get_combination_count(index_count as usize, 8);
    let mut combinations: Vec<[u8; 8]> = Vec::with_capacity(combination_count);
    let mut live_array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 6];
    for _ in 0..combination_count {
        live_array[7] = live_array[7] + 1;
        if live_array[7] == index_count {
            array_flipper(&mut live_array);
        };
        combinations.push(live_array.clone());
    };
    combinations
}

fn array_flipper(array: &mut [u8; 8]) {
    let mut flip_index = 7;
    let mut top_allowed = array[flip_index] - 1;
    for i in 0..8 {
        let ia = 7-i;
        if array[ia] >= top_allowed {
            flip_index -= 1;
            top_allowed -= 1;
        };
    };
    let mut low = 0u8;
    for i in 0..8 {
        if i == flip_index {
            low = array[i] + 1;
            array[i] = low;
        } else if i > flip_index {
            low += 1;
            array[i] = low;
        };
    };
}

fn get_combination_count(unique_elements: usize, combination_length: usize) -> usize {
    if combination_length > unique_elements {
        return 0;
    };
    let mut result = 1;
    for i in 1..=combination_length {
        result = result * (unique_elements - i + 1) / i;
    };
    result
}

const ILLEGAL_PAIRS: [(u8, u8); 11] = [
    (5, 22),  // Aptitude
    (6, 28),  // Chamber
    (4, 16),  // Point Strike
    (2, 7),   // Scope
    (1, 26),  // Serration
    (17, 6),  // Acuity exclude Galvanized Chamber
    (17, 28), // Acuity exclude Split Chamber
    (17, 6),  // Acuity exclude Vigilante Armaments
    (25, 20), // Cannonade exclude Primed Shred
    (25, 27), // Cannonade exclude Speed Trigger
    (25, 32), // Cannonade exclude Vile Acceleration
];
const MAX_INDEX: usize = 35;

pub fn filter_combinations(
    combinations: &mut Vec<[u8; 8]>, required: &[u8], disallowed: &[u8]
) {
    let required_mask = build_mask(required);
    let disallowed_mask = build_mask(disallowed);
    combinations.retain(|combo: &[u8; 8]| keep_combo_bitmask(combo, required_mask, disallowed_mask));
}

#[inline(always)]
pub fn build_mask(indices: &[u8]) -> u64 {
    let mut mask: u64 = 0;
    for &i in indices {
        mask |= 1 << i;
    };
    mask
}

#[inline(always)]
fn keep_combo_bitmask(combo: &[u8; 8], required_mask: u64, disallowed_mask: u64) -> bool {
    // create bitmask
    let mut bits: u64 = 0;
    for &i in combo.iter() {
        bits |= 1 << i;
    };
    
    // filter illegal mod pairs
    for (a, b) in ILLEGAL_PAIRS {
        if (bits & (1 << a)) != 0 && (bits & (1 << b)) != 0 {
            return false;
        };
    };
    
    if (bits & required_mask) != required_mask {
        return false;
    };
    
    if (bits & disallowed_mask) != 0 {
        return false;
    };
    
    return true;
}

pub fn test_all_builds(
    combinations: &Vec<[u8; 8]>, 
    base_gun_stats: &GunStats, 
    criteria: &Criteria, 
    loaded_mods: &Vec<WeaponMod>, 
    loaded_arcanes: &Vec<WeaponMod>
) -> Vec<WeaponReport> {
    let mut builds: Vec<WeaponReport> = Vec::with_capacity(combinations.len() * loaded_arcanes.iter().len());
    for combo in combinations {
        let modded_sums = GunStatModSums::from_mod_list(
            combo,
            loaded_mods,
            base_gun_stats,
            criteria
        );
        for (i, arcane) in loaded_arcanes.iter().enumerate() {
            let mut arcane_sums = modded_sums.clone();
            arcane_sums.add_mod(arcane, criteria.kills(), base_gun_stats.semi);
            let arcane_stats = base_gun_stats.apply_stat_sums(&arcane_sums);
            let arcane_report = arcane_stats.generate_report(criteria.clone(), combo, i as u8);
            builds.push(arcane_report);
        };
    };
    return builds;
}

pub fn sort_by_criteria(reports: &mut Vec<WeaponReport>, criteria: Criteria) {
    match criteria {
        Criteria::PerShot | Criteria::PerShotNoKills => {
            reports.sort_by_key(|r| Reverse(r.hit_damage));
        },
        Criteria::BurstDPS | Criteria::BurstDPSNoKills => {
            reports.sort_by_key(|r| Reverse(r.burst_dps));
        },
        Criteria::SustainedDPS | Criteria::SustainedDPSNoKills => {
            reports.sort_by_key(|r| Reverse(r.sustained_dps));
        }
    };
}