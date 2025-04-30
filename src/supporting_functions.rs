use std::io::stdin;
use crate::mod_structs::*;
use crate::weapon_structs::*;

pub fn weapon_select_loop() -> GunStats {
    let mut selected = false;
    let mut gun_stats = GunStats::EMPTY_GUN;
    while !selected {
        println!("Please enter a weapon name.");
        let mut input = take_input(
            "Or, press enter for a list of supported weapons:"
        );
        gun_stats = if input.len() > 0 {
            println!("Looking up '{}'...", input);
            GunStats::gun_lookup(input.as_str())
        } else {
            println!("Using numbered table...");
            use_weapon_list()
        };
        selected = gun_stats.fire_rate != 0.0;
    };
    return gun_stats;
}

fn use_weapon_list() -> GunStats {
    println!();
    println!("Enter the number that corresponds with your weapon:");
    for (index, weapon_name) in GunStats::RIFLE_LIST.iter().enumerate() {
        println!("{}: {}", index, weapon_name)
    };
    let input = take_input("Leave blank, or fuck up the input to go back:");
    
    return if let Ok(index) = input.parse::<usize>() {
        GunStats::gun_lookup(GunStats::RIFLE_LIST[index])
    } else {
        GunStats::EMPTY_GUN
    };
}

pub fn take_input(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    let _ = stdin().read_line(&mut buffer);
    buffer.pop();
    return buffer;
}

fn try_mod(
    mod_sum: &GunStatModSums, base_stats: &GunStats, weapon_mod: &WeaponMod, criteria: &Criteria
) -> f32 {  // this should return a number representing the effective multiplier the mod applies
    let old_stats = base_stats.apply_stat_sums(&mod_sum);
    let mut new_mod_sum = mod_sum.clone();
    new_mod_sum.add_mod(weapon_mod, criteria.kills(), base_stats.semi);
    let new_stats = base_stats.apply_stat_sums(&new_mod_sum);
    return compare_stats(&old_stats, &new_stats, criteria);
}

fn compare_stats(
    old_stats: &GunStats, new_stats: &GunStats, criteria: &Criteria
) -> f32 {
    let old_shot_damage = old_stats.calculate_shot_damage();
    let new_shot_damage = new_stats.calculate_shot_damage();
    if criteria == &Criteria::PerShot || criteria == &Criteria::PerShotNoKills {
        return new_shot_damage / old_shot_damage;
    };
    let old_burst_damage = old_stats.calculate_burst_dps(old_shot_damage);
    let new_burst_damage = new_stats.calculate_burst_dps(new_shot_damage);
    if criteria == &Criteria::BurstDPS || criteria == &Criteria::BurstDPSNoKills {
        return new_burst_damage / old_burst_damage;
    };
    let old_sustained_damage = old_stats.calculate_sustained_dps(old_burst_damage);
    let new_sustained_damage = new_stats.calculate_sustained_dps(new_burst_damage);
    return new_sustained_damage / old_sustained_damage;
}

fn check_rifle_mods(base_gun_stats: &GunStats, mod_list: &ModList) -> i8 {
    let current_mod_sum = GunStatModSums::from_mod_list(mod_list, base_gun_stats);
    let current_gun_stats = base_gun_stats.apply_stat_sums(&current_mod_sum);
    let mut high_score = 1.0f32;
    let mut high_index = &-1i8;
    for array_of_mod_indexes in RifleMods::MOD_INDEXES_REFERENCES {
        for mod_index in array_of_mod_indexes {
            if mod_list.index_array.contains(mod_index) {
                continue;
            };
            let weapon_mod = &RifleMods::ALL_MODS[mod_index.clone() as usize];
            let mut modded_sum = current_mod_sum.clone();
            modded_sum.add_mod(weapon_mod, mod_list.criteria.kills(), current_gun_stats.semi);
            let modded_stats = base_gun_stats.apply_stat_sums(&modded_sum);
            let new_score = compare_stats(&current_gun_stats, &modded_stats, &mod_list.criteria);
            if new_score > high_score {
                high_score = new_score;
                high_index = mod_index;
            };
        };
    };
    println!("Adding {}, scored {}...",
             &RifleMods::ALL_MODS[high_index.clone() as usize].name, high_score
    );
    return high_index.clone();
}

fn find_next_best_mod_index(base_gun_stats: &GunStats, mod_list: &ModList) -> i8 {
    return match base_gun_stats.gun_type {
        GunType::Rifle => {
            check_rifle_mods(base_gun_stats, mod_list)
        }
    };
}

pub fn fill_empty_mod_slots(
    mod_list: &mut ModList, base_gun_stats: &GunStats
) -> f32 {
    for i in 0..mod_list.index_array.len() {
        mod_list.index_array[i] = find_next_best_mod_index(base_gun_stats, &mod_list);
    };
    let new_sums = GunStatModSums::from_mod_list(mod_list, &base_gun_stats);
    let new_stats = base_gun_stats.apply_stat_sums(&new_sums);
    compare_stats(base_gun_stats, &new_stats, &mod_list.criteria)
}

fn identify_weakest_mod_slot(mod_list: &ModList, base_gun_stats: &GunStats) -> i8 {
    let current_mod_sum = GunStatModSums::from_mod_list(&mod_list, &base_gun_stats);
    let current_weapon_stats = base_gun_stats.apply_stat_sums(&current_mod_sum);
    let mut weakest_score = 1000.0;
    let mut weak_index: i8 = -1;
    for (slot_index, mod_index) in mod_list.index_array.iter().enumerate() {
        let weapon_mod = lookup_mod(&base_gun_stats.gun_type, mod_index.clone() as usize);
        let mut modified_sum = current_mod_sum.clone();
        modified_sum.remove_mod(weapon_mod, mod_list.criteria.kills(), base_gun_stats.semi);
        let modified_weapon_stats = base_gun_stats.apply_stat_sums(&modified_sum);
        let mod_rating = compare_stats(&modified_weapon_stats, &current_weapon_stats, &mod_list.criteria);
        if mod_rating < weakest_score {
            weakest_score = mod_rating;
            weak_index = slot_index as i8;
        };
    };
    return weak_index;
}

// fn test_for_better_mods(&mut mod_list: &ModList, base_gun_stats: &GunStats) -> {
// 
// }