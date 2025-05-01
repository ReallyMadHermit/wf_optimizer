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
