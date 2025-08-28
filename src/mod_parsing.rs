use crate::combinatorics::{generate_combinations, BuildCombo};
use crate::data::{GUN_MODS, GUN_ARCANES};
use crate::context_core::{ModdingContext, WeaponType};

const BEHAVIOR_SLICE_INDICES: [usize;2] = [6, 11];
const BSI: [usize;2] = BEHAVIOR_SLICE_INDICES;
const MOD_DATA_SLICE_INDICES: [usize;2] = [2, 5];
const MDSI: [usize;2] = MOD_DATA_SLICE_INDICES;

pub struct LoadedMods {
    mod_names: Vec<String>,
    mod_data: Vec<ModData>,
    pub combinations: Vec<BuildCombo>,  // TODO: replace included mods with combinations from combinatorics
    pub mod_count: u8,  // TODO: filter illegal pairs inside of loaded mods
    pub arcane_count: u8
} impl LoadedMods {

    pub fn new(modding_context: &ModdingContext) -> Self {
        let mod_lines: Vec<&str> = GUN_MODS.lines().collect();
        let arcane_lines: Vec<&str> = GUN_ARCANES.lines().collect();
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
        Self::parse_mods(&mut loaded_mods, &mod_range, mod_scores, false);
        Self::parse_mods(&mut loaded_mods, &arcane_range, arcane_scores, true);
        loaded_mods.calculate_combinatorics();  // TODO write filtration
        loaded_mods
    }

    // TODO: write a get_many(&[u8;8]) -> Option<[ModData; 8]>
    pub fn get_mod(&self, mod_id: u8) -> ModData {
        self.mod_data[mod_id as usize]
    }

    // TODO: write a get_many(&[u8;8]) -> Option<[&str; 8]>
    pub fn get_name(&self, mod_id: u8) -> &str {
        &self.mod_names[mod_id as usize]
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
    AcuityBonus,
    StatusDamage,
    PunchThrough,
    AmmoEfficiency,
    Riven
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModData {  // TODO: impliment riven parsing
    pub stat_type_1: ModStatType,
    pub stat_type_2: ModStatType,
    pub stat_value_1: i16,
    pub stat_value_2: i16
}

// private block
impl LoadedMods {

    fn empty(size: usize) -> Self {
        Self {
            mod_names: Vec::with_capacity(size),
            mod_data: Vec::with_capacity(size),
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
        mod_name: &str,
        mod_data: ModData,
        arcane: bool
    ) -> u8 {
        self.mod_names.push(String::from(mod_name));
        self.mod_data.push(mod_data);
        if arcane {
            self.arcane_count += 1;
        } else {
            self.mod_count += 1;
        };
        self.len() as u8 - 1
    }

    // fn include_mod(&mut self, mod_id: u8) {
    //     let i = self.included_mods[0].wrapping_add(1);
    //     self.included_mods[0] = i;
    //     self.included_mods[i as usize] = mod_id;
    // }

    fn parse_mods(loaded_mods: &mut LoadedMods, lines: &[&str], scores: Vec<i8>, arcane: bool) {
        for (&line, &score) in lines.iter().zip(scores.iter()) {
            if score < 0 {
                continue;
            };
            let split: Vec<&str> = line.split(",").collect();
            let data = ModData::from_split_slice(&split[MDSI[0]..=MDSI[1]]);
            let mod_id = loaded_mods.load_mod(split[1], data, arcane);
            // if score > 0 {
            //     loaded_mods.include_mod(mod_id)
            // };
        };
    }

    fn should_include(csv_line: &str, modding_context: &ModdingContext) -> i8 {
        let split: Vec<&str> = csv_line.split(",").collect();
        if !WeaponType::is_compatible(modding_context.weapon_type, WeaponType::from_str(split[0])) { return -1 };
        return Self::context_test(&split[BSI[0]..=BSI[1]], modding_context);
    }

    fn context_test(behavior_slice: &[&str], modding_context: &ModdingContext) -> i8 {
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

    fn generate_illegal_pairs() -> Option<Vec<(u8, u8)>> {  // TODO: write the illegal pair filter
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

}

impl ModStatType {

    fn from_str(string_slice: &str) -> Self {
        return match string_slice {
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
            "AcuityBonus" => Self::AcuityBonus,
            "StatusDamage" => Self::StatusDamage,
            "PunchThrough" => Self::PunchThrough,
            "AmmoEfficiency" => Self::AmmoEfficiency,
            "Riven" => Self::Riven,
            _ => {
                println!("{} not found! Using 'None'", string_slice);
                Self::None
            }
        };
    }

    fn from_riven_str(s: &str) -> Self {
        match s {
            "C" => Self::Cold,
            "CC" => Self::CritChance,
            "CD" => Self::CritDamage,
            "D" => Self::Damage,
            "E" => Self::Shock,
            "H" => Self::Heat,
            "F" => Self::FireRate,
            "MG" => Self::MagazineCapacity,
            "MS" => Self::Multishot,
            "T" => Self::Toxic,
            "R" => Self::ReloadSpeed,
            "S" => Self::StatusChance,
            _ => Self::None
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::None => "None",
            Self::Damage => "Damage",
            Self::Heat => "Heat",
            Self::Cold => "Cold",
            Self::Toxic => "Toxic",
            Self::Shock => "Shock",
            Self::Magnetic => "Magnetic",
            Self::Radiation => "Radiation",
            Self::Multishot => "Multishot",
            Self::CritChance => "Crit Chance",
            Self::CritDamage => "Crit Damage",
            Self::FireRate => "Firerate",
            Self::StatusChance => "Status Chance",
            Self::ConditionOverload => "Condition Overload",
            Self::MagazineCapacity => "Magazine Capacity",
            Self::ReloadSpeed => "Reload Speed",
            Self::AcuityBonus => "Acuity Bonus",
            Self::StatusDamage => "Status Damage",
            Self::PunchThrough => "Punch Through",
            Self::AmmoEfficiency => "Ammo Efficiency",
            Self::Riven => "Riven"
        }
    }

}

impl ModData {

    fn from_split_slice(slice: &[&str]) -> Self {
        let stat_type_1 = ModStatType::from_str(slice[0]);
        let stat_value_1: i16 = if let Ok(parsed) = slice[1].parse() {
            parsed
        } else {
            0
        };
        let stat_type_2 = ModStatType::from_str(slice[2]);
        let stat_value_2: i16 = if let Ok(parsed) = slice[3].parse() {
            parsed
        } else {
            0
        };
        Self {
            stat_type_1,
            stat_type_2,
            stat_value_1,
            stat_value_2
        }
    }

}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ModBehavior {
    Exclude,
    Include,
    Parallel,
    NotExclude,
    NotInclude,
    NotParallel,
    NothingSpecial
} impl ModBehavior {

    fn from_str(mod_behavior_str: &str) -> Self {
        match mod_behavior_str {
            "EXC"  => Self::Exclude,
            "INC"  => Self::Include,
            "PAR"  => Self::Parallel,
            "!EXC" => Self::NotExclude,
            "!INC" => Self::NotExclude,
            "!PAR" => Self::NotParallel,
            _      => Self::NothingSpecial
        }
    }

}