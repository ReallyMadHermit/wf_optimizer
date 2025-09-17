use crate::combinatorics::{generate_combinations, BuildCombo};
use crate::data::{GUN_MODS, GUN_ARCANES};
use crate::context_core::{ModdingContext, WeaponType};

const BEHAVIOR_SLICE_INDICES: [usize;2] = [6, 12];
const BSI: [usize;2] = BEHAVIOR_SLICE_INDICES;
const MOD_DATA_SLICE_INDICES: [usize;2] = [2, 5];
const MDSI: [usize;2] = MOD_DATA_SLICE_INDICES;
const BADMATCH_INDEX: usize = 13;

pub struct LoadedMods {
    mod_names: Vec<&'static str>,
    mod_data: Vec<ModData>,
    included_mods: Option<Vec<u8>>,
    pub combinations: Vec<BuildCombo>,
    pub mod_count: u8,
    pub arcane_count: u8
} impl LoadedMods {

    pub fn new(modding_context: &ModdingContext) -> Self {
        let mod_lines: Vec<&'static str> = GUN_MODS.lines().collect();
        let arcane_lines: Vec<&'static str> = GUN_ARCANES.lines().collect();
        let mod_range = &mod_lines[1..];
        let arcane_range = &arcane_lines[1..];
        let mut mod_scores: Vec<i8> = Vec::with_capacity(mod_range.len());
        let mut arcane_scores: Vec<i8> = Vec::with_capacity(arcane_range.len());
        let mut size = 0usize;
        for (&lines, scores) in [
            (&mod_range, &mut mod_scores),
            (&arcane_range, &mut arcane_scores)
        ] {
            for &line in lines {
                let score = Self::should_include(line, modding_context);
                if score >= 0 {size+=1};
                scores.push(score);
            };
        };
        let mut loaded_mods = LoadedMods::empty(size);
        Self::parse_mods(&mut loaded_mods, mod_range, mod_scores, false);
        Self::parse_mods(&mut loaded_mods, arcane_range, arcane_scores, true);
        loaded_mods.calculate_combinatorics();
        loaded_mods.filter_loaded_mods(modding_context);
        loaded_mods
    }

    pub fn get_data(&self, mod_id: u8) -> &[(ModStatType, i16)] {
        self.mod_data[mod_id as usize].get()
    }

    pub fn get_name(&self, mod_id: u8) -> &str {
        self.mod_names[mod_id as usize]
    }

}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ModStatType {  // TODO: represent pistol arcanes (somehow)
    None,
    Damage,
    Heat,
    Cold,
    Toxic,
    Shock,
    Magnetic,
    Radiation,
    Multishot,
    CritChance,
    CritDamage,
    FireRate,
    StatusChance,
    ConditionOverload,
    MagazineCapacity,
    ReloadSpeed,
    StatusDamage,
    PunchThrough,
    AmmoEfficiency,
    Headshot,
    Riven
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModData {
    stats: [(ModStatType, i16); 4],
    count: u8
} impl ModData {
    pub fn get(&self) -> &[(ModStatType, i16)] {
        &self.stats[0..self.count as usize]
    }
}

pub struct RivenMod {
    pub stats: Vec<(ModStatType, i16)>
} impl RivenMod {

    pub fn from_str(input: &str) -> Option<Self> {
        let upper = input.to_ascii_uppercase();
        let mut stat_type = ModStatType::None;
        let mut stat_value = 0i16;
        let mut stats = Vec::with_capacity(4);
        let mut value_flag = false;
        let mut type_flag = false;
        for s in upper.split(" ") {
            if let Ok(i) = s.parse() {
                stat_value = i;
                value_flag = true;
            } else {
                stat_type = ModStatType::from_riven_str(s);
                type_flag = true;
            };
            if value_flag & type_flag {
                stats.push((stat_type, stat_value));
                value_flag = false;
                type_flag = false;
            };
        };
        if !stats.is_empty() {
            Some(
                Self {
                    stats
                }
            )
        } else {
            None
        }
    }

    pub fn println_stats(&self) {
        println!("Your riven stats are:");
        for &(stat_type, stat_value) in &self.stats {
            if stat_value > 0 {
                println!("+{}% {}", stat_value, stat_type.to_str());
            } else {
                println!("-{}% {}", stat_value.abs(), stat_type.to_str());
            };
        };
    }

    pub fn show_riven_key() {
        println!("Use the below stat-keys and a number for each value to describe your rolls:");
        println!("D: Damage");
        println!("MS: Multi-Shot");
        println!("-");
        println!("CC: CritChance");
        println!("CD: Crit Damage");
        println!("-");
        println!("C: Cold");
        println!("E: Electricity");
        println!("H: Heat");
        println!("T: Toxic");
        println!("SC: Status Chance");
        println!("-");
        println!("FR: Fire-Rate");
        println!("MC: Magazine Capacity");
        println!("RS: Reload Speed");
        println!("-");
        println!("Some examples of valid combinations:");
        println!("140 D 80 T -20 CC");
        println!("200 C -80 FR");
        println!("CC 140 CD 150 D -60");
        println!("As long as you alternate between key and values, they can be in either order.");
    }

}

// private block
impl LoadedMods {

    fn empty(size: usize) -> Self {
        Self {
            mod_names: Vec::with_capacity(size),
            mod_data: Vec::with_capacity(size),
            included_mods: None,
            combinations: Vec::new(),
            mod_count: 0,
            arcane_count: 0
        }
    }

    fn calculate_combinatorics(&mut self) {
        self.combinations = generate_combinations(self.mod_count, self.arcane_count);
    }

    fn len(&self) -> usize {
        self.mod_data.len()
    }

    fn load_mod(
        &mut self,
        mod_name: &'static str,
        mod_data: ModData,
        arcane: bool
    ) -> u8 {
        self.mod_names.push(mod_name);
        self.mod_data.push(mod_data);
        if arcane {
            self.arcane_count += 1;
        } else {
            self.mod_count += 1;
        };
        self.len() as u8 - 1
    }

    fn include_mod(&mut self, mod_id: u8) {
        match &mut self.included_mods {
            Some(ref mut vec) => vec.push(mod_id),
            None => {
                let mut vec = Vec::with_capacity(3);
                vec.push(mod_id);
                self.included_mods = Some(vec);
            }
        }
    }

    fn parse_mods(loaded_mods: &mut LoadedMods, lines: &[&'static str], scores: Vec<i8>, arcane: bool) {
        for (&line, &score) in lines.iter().zip(scores.iter()) {
            if score < 0 {
                continue;
            };
            let split: Vec<&str> = line.split(",").collect();
            let data = ModData::from_split_slice(&split[MDSI[0]..=MDSI[1]]);
            let mod_id = loaded_mods.load_mod(split[1], data, arcane);
            if score > 0 {
                loaded_mods.include_mod(mod_id)
            };
        };
    }

    fn should_include(csv_line: &str, modding_context: &ModdingContext) -> i8 {
        let split: Vec<&str> = csv_line.split(",").collect();
        if !WeaponType::is_compatible(modding_context.weapon_type, WeaponType::from_str(split[0])) { return -1 };
        Self::context_test(&split[BSI[0]..=BSI[1]], modding_context)
    }

    fn context_test(behavior_slice: &[&str], modding_context: &ModdingContext) -> i8 {
        let mut include = false;
        let kills_behavior = ModBehavior::from_str(behavior_slice[0]);
        let aiming_behavior = ModBehavior::from_str(behavior_slice[1]);
        let headshot_behavior = ModBehavior::from_str(behavior_slice[2]);
        let semi_behavior = ModBehavior::from_str(behavior_slice[3]);
        let acuity_behavior = ModBehavior::from_str(behavior_slice[4]);
        let amalgam_behavior = ModBehavior::from_str(behavior_slice[5]);
        let riven_behavior = ModBehavior::from_str(behavior_slice[6]);
        for (behavior, truth) in [
            (kills_behavior, modding_context.kills),
            (aiming_behavior, modding_context.aiming),
            (headshot_behavior, modding_context.headshot),
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
                (false, ModBehavior::NotParallel) => include = true,
                _ => continue
            };
        };
        if include {
            1
        } else {
            0
        }
    }

    fn filter_loaded_mods(&mut self, modding_context: &ModdingContext) {
        let illegals = Self::generate_illegal_pairs(modding_context);
        let pairs: Vec<(u8, u8)> = if let Some(name_pairs) = illegals {
             self.lookup_illegal_pairs(name_pairs)
        } else {
            return;
        };
        if let Some(included) = &self.included_mods {
            self.combinations.retain(|combo| Self::contains_required_mods(&combo.mod_combo, included))
        };
        self.combinations.retain(|combo| !Self::contains_illegal_pair(&combo.mod_combo, &pairs));
        self.combinations.shrink_to_fit();
    }

    fn contains_required_mods(combo: &[u8; 8], included_mods: &[u8]) -> bool {
        let mut flag_array = [false; 64];
        for &i in combo {
            flag_array[i as usize] = true;
        };
        included_mods.iter().all(|&i| flag_array[i as usize])
    }

    fn contains_illegal_pair(combo: &[u8; 8], illegal_pairs: &[(u8, u8)]) -> bool {
        let mut flag_array = [false; 64];
        for &i in combo {
            flag_array[i as usize] = true;
        };
        illegal_pairs.iter().any(|&(a, b)| flag_array[a as usize] && flag_array[b as usize])
    }

    fn lookup_illegal_pairs(&self, name_pairs: Vec<(&str, &str)>) -> Vec<(u8, u8)> {
        let mut results: Vec<(u8, u8)> = Vec::with_capacity(name_pairs.len());
        for &(name_a, name_b) in name_pairs.iter() {
            let mut match_a: Option<u8> = None;
            let mut match_b: Option<u8> = None;
            for (i, &name) in self.mod_names.iter().enumerate() {
                if name == name_a {
                    match_a = Some(i as u8);
                } else if name == name_b {
                    match_b = Some(i as u8);
                };
                if match_a.is_some() && match_b.is_some() {
                    let a = Option::unwrap(match_a);
                    let b = Option::unwrap(match_b);
                    let pair = (a.min(b), b.max(a));
                    if !results.contains(&pair) {
                        results.push(pair);
                    };
                    break;
                };
            };
        };
        results
    }

    // TODO: write this to allow multiple BADMATCH mods on a single line, filtration further down prevents dupes
    fn generate_illegal_pairs(modding_context: &ModdingContext) -> Option<Vec<(&'static str, &'static str)>> {
        let weapon_type = modding_context.weapon_type;
        let mut pairs: Vec<(&str, &str)> = Vec::with_capacity(4);
        for &row in &GUN_MODS.lines().collect::<Vec<&str>>()[1..] {
            let s: Vec<&str> = row.split(",").collect();
            let mod_type = WeaponType::from_str(s[0]);
            if !WeaponType::is_compatible(weapon_type, mod_type) {
                continue;
            };
            if !s[BADMATCH_INDEX].is_empty() {
                pairs.push((
                    s[1],
                    s[BADMATCH_INDEX]
                ));
            };
        };
        if !pairs.is_empty() {
            pairs.shrink_to_fit();
            Some(pairs)
        } else {
            None
        }
    }

}

impl ModStatType {  // TODO: re-add Acuity stat and cannonade to lock out multishot and firerate instead of just excluding the mods

    fn from_str(string_slice: &str) -> Self {
        match string_slice {
            "None" => Self::None,
            "Damage" => Self::Damage,
            "Heat" => Self::Heat,
            "Cold" => Self::Cold,
            "Toxic" => Self::Toxic,
            "Shock" => Self::Shock,
            "Magnetic" => Self::Magnetic,
            "Radiation" => Self::Radiation,
            "Multishot" => Self::Multishot,
            "CritChance" => Self::CritChance,
            "CritDamage" => Self::CritDamage,
            "FireRate" => Self::FireRate,
            "StatusChance" => Self::StatusChance,
            "ConditionOverload" => Self::ConditionOverload,
            "MagazineCapacity" => Self::MagazineCapacity,
            "ReloadSpeed" => Self::ReloadSpeed,
            "StatusDamage" => Self::StatusDamage,
            "PunchThrough" => Self::PunchThrough,
            "AmmoEfficiency" => Self::AmmoEfficiency,
            "Headshot" => Self::Headshot,
            "Riven" => Self::Riven,
            _ => {
                println!("{} not found! Using 'None'", string_slice);
                Self::None
            }
        }
    }

    fn from_riven_str(s: &str) -> Self {
        match s {
            "C" => Self::Cold,
            "CC" => Self::CritChance,
            "CD" => Self::CritDamage,
            "D" => Self::Damage,
            "E" => Self::Shock,
            "H" => Self::Heat,
            "FR" => Self::FireRate,
            "MC" => Self::MagazineCapacity,
            "MS" => Self::Multishot,
            "T" => Self::Toxic,
            "RS" => Self::ReloadSpeed,
            "SC" => Self::StatusChance,
            _ => Self::None
        }
    }

    fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Damage => "Damage",
            Self::Heat => "Heat",
            Self::Cold => "Cold",
            Self::Toxic => "Toxic",
            Self::Shock => "Electricity",
            Self::Magnetic => "Magnetic",
            Self::Radiation => "Radiation",
            Self::Multishot => "Multi-Shot",
            Self::CritChance => "Crit Chance",
            Self::CritDamage => "Crit Damage",
            Self::FireRate => "Fire-Rate",
            Self::StatusChance => "Status Chance",
            Self::ConditionOverload => "Condition Overload",
            Self::MagazineCapacity => "Magazine Capacity",
            Self::ReloadSpeed => "Reload Speed",
            Self::StatusDamage => "Status Damage",
            Self::PunchThrough => "Punch Through",
            Self::AmmoEfficiency => "Ammo Efficiency",
            Self::Headshot => "Headshot Damage",
            Self::Riven => "Riven"
        }
    }

}

impl ModData {

    fn empty() -> Self {
        Self {
            stats: [(ModStatType::None, 0); 4],
            count: 0
        }
    }

    fn from_split_slice(slice: &[&str]) -> Self {
        let mut mod_data = Self::empty();
        let stat_type_1 = ModStatType::from_str(slice[0]);
        let stat_value_1: i16 = slice[1].parse().unwrap_or_default();
        let stat_type_2 = ModStatType::from_str(slice[2]);
        let stat_value_2: i16 = slice[3].parse().unwrap_or_default();
        mod_data.push(stat_type_1, stat_value_1);
        mod_data.push(stat_type_2, stat_value_2);
        mod_data
    }

    fn push(&mut self, mod_stat_type: ModStatType, value: i16) {
        self.stats[self.count as usize] = (mod_stat_type, value);
        self.count += 1;
    }

}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ModBehavior {
    Exclude,
    Include,
    Parallel,
    NotExclude,
    NotParallel,
    NothingSpecial
} impl ModBehavior {

    fn from_str(mod_behavior_str: &str) -> Self {
        match mod_behavior_str {
            "EXC"  => Self::Exclude,
            "INC"  => Self::Include,
            "PAR"  => Self::Parallel,
            "!EXC" => Self::NotExclude,
            "!PAR" => Self::NotParallel,
            _      => {
                println!("ERR: no match for '{}' ModBehavior", mod_behavior_str);
                Self::NothingSpecial
            }
        }
    }

}