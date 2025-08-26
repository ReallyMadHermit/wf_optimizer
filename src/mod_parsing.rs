use crate::data::{GUN_MODS, GUN_ARCANES};
use crate::structs::{GunModData, GunModdingContext, GunType, LoadedGunMods, ModBehavior};

const BEHAVIOR_SLICE_INDICES: [usize;2] = [6, 11];
const BSI: [usize;2] = BEHAVIOR_SLICE_INDICES;
const MOD_DATA_SLICE_INDICES: [usize;2] = [2, 5];
const MDSI: [usize;2] = MOD_DATA_SLICE_INDICES;

pub fn load_gun_mods(modding_context: &GunModdingContext) -> LoadedGunMods {
    let mut mod_lines: Vec<&str> = GUN_MODS.lines().collect();
    let mut arcane_lines: Vec<&str> = GUN_ARCANES.lines().collect();
    let mod_range = &mod_lines[1..];
    let arcane_range = &arcane_lines[1..];
    let mut mod_scores: Vec<i8> = Vec::with_capacity(mod_range.len());
    let mut arcane_scores: Vec<i8> = Vec::with_capacity(arcane_range.len());
    let mut size = 0usize;
    for (&lines, scores) in [
        (&mod_range, &mut mod_scores),
        (&mod_range, &mut arcane_scores)
    ] {
        for &line in lines {
            let score = score_inclusion(line, modding_context);
            if score >= 0 {size+=1};
            scores.push(score);
        };
    };
    let mut loaded_mods = LoadedGunMods::new(size);
    parse_mods(&mut loaded_mods, &mod_range, mod_scores, false);
    parse_mods(&mut loaded_mods, &arcane_range, arcane_scores, true);
    loaded_mods
}

fn parse_mods(loaded_mods: &mut LoadedGunMods, lines: &[&str], scores: Vec<i8>, arcane: bool) {
    for (&line, &score) in lines.iter().zip(scores.iter()) {
        if score < 0 {
            continue;
        };
        let split: Vec<&str> = line.split(",").collect();
        let data = GunModData::from_split_slice(&split[MDSI[0]..=MDSI[1]]);
        let mod_id = loaded_mods.load_mod(split[1], data, arcane);
        if score > 0 {
            loaded_mods.include_mod(mod_id)
        };
    };
}

fn score_inclusion(csv_line: &str, modding_context: &GunModdingContext) -> i8 {
    let split: Vec<&str> = csv_line.split(",").collect();
    if !GunType::is_compatible(modding_context.gun_type, GunType::from_str(split[0])) { return -1 };
    return score_context(&split[BSI[0]..=BSI[1]], modding_context);
}

fn score_context(behavior_slice: &[&str], modding_context: &GunModdingContext) -> i8 {
    let mut include = false;
    let kills_behavior = ModBehavior::from_str(behavior_slice[0]);
    let aiming_behavior = ModBehavior::from_str(behavior_slice[1]);
    let semi_behavior = ModBehavior::from_str(behavior_slice[2]);
    let acuity_behavior = ModBehavior::from_str(behavior_slice[3]);
    let amalgam_behavior = ModBehavior::from_str(behavior_slice[4]);
    let riven_behavior = ModBehavior::from_str(behavior_slice[5]);
    for (behavior, truth) in [
        (kills_behavior, modding_context.kills),
        (aiming_behavior, modding_context.aiming),
        (semi_behavior, modding_context.semi),
        (acuity_behavior, modding_context.acuity),
        (amalgam_behavior, modding_context.prefer_amalgam),
        (riven_behavior, modding_context.riven)
    ] {
        match (truth, behavior) {
            (_, ModBehavior::NothingSpecial) => continue,
            (true, ModBehavior::Exclude | ModBehavior::NotParallel) |
            (false, ModBehavior::NotExclude | ModBehavior::Parallel) => return -1,
            (true, ModBehavior::Include | ModBehavior::Parallel) |
            (false, ModBehavior::NotInclude | ModBehavior::NotParallel) => include = true,
            _ => continue
        };
    };
    if include {
        1
    } else {
        0
    }
}

pub fn generate_illegal_pairs() -> Option<Vec<(u8, u8)>> {  // todo this lmao
    let mut pairs: Vec<(u8, u8)> = Vec::with_capacity(3);
    for &row in &GUN_MODS.lines().collect::<Vec<&str>>()[1..] {
        let s: Vec<&str> = row.split(",").collect();
        if s[12] != "" {
            println!("{}, {}", s[1], s[12]);
        };
    };
    if pairs.len() > 0 {
        Option::Some(pairs)
    } else {
        Option::None
    }
}