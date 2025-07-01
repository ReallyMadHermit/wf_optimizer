use crate::brute_force_solution::{filter_combinations, generate_combinations};
use crate::cli_inputs::{establish_the_facts, loop_integer_prompt, yes_no_prompt};
use crate::file_interfacing::load_mods;
use crate::mod_structs::{GunModSums, LoadedGunMods};
use crate::weapon_structs::{GunData, GunStats, GunType};
use std::fmt::Write;
use std::time::Instant;

#[derive(Clone, Eq, PartialEq)]
pub struct ModdingContext {
    pub gun_type: GunType,
    pub damage: DamageCriteria,
    pub kills: bool,
    pub aiming: bool,
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool
} impl ModdingContext {

    pub fn interview_user(gun_type: GunType, semi: bool) -> Self {
        let damage = DamageCriteria::determine_criteria();
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
        ModdingContext {
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
pub enum DamageCriteria {
    PerShot,
    BurstDPS,
    SustainedDPS
} impl DamageCriteria {

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

#[derive(Copy, Clone)]
pub struct SortingHelper {
    pub sorting_integer: u32,
    pub combination_index: u32
} impl SortingHelper {
    
    pub fn new(sorting_value: f32, combination_index: usize) -> Self {
        Self {
            sorting_integer: u32::MAX - sorting_value as u32,
            combination_index: combination_index as u32
        }
    }
    
}

pub struct ReportAggregator {
    gun_data: GunData,
    loaded_mods: LoadedGunMods,
    combinations: Vec<u64>,
    modded_sums: Vec<GunModSums>,
    modded_stats: Vec<GunStats>,
    shot_damage: Vec<f32>,
    burst_damage: Vec<f32>,
    sustained_damage: Vec<f32>,
    shot_sorting: Vec<SortingHelper>,
    burst_sorting: Vec<SortingHelper>,
    sustained_sorting: Vec<SortingHelper>
} impl ReportAggregator {
    
    pub fn new(selected_gun: GunData, gun_modding_context: ModdingContext) -> Self {
        let start = Instant::now();
        println!("Loading mods... ({:?}s elapsed)", start.elapsed());
        let loaded_mods = load_mods(&gun_modding_context);
        println!("Generating combinations... ({:?}s elapsed)", start.elapsed());
        let mut combinations = generate_combinations(&loaded_mods);
        println!("Filtering combinations... ({:?}s elapsed)", start.elapsed());
        filter_combinations(&mut combinations, loaded_mods.included_mods_slice());
        combinations.shrink_to_fit();
        let count = combinations.len();
        let mut report_ag = ReportAggregator {
            gun_data: selected_gun,
            loaded_mods,
            combinations,
            modded_sums: Vec::with_capacity(count),
            modded_stats: Vec::with_capacity(count),
            shot_damage: Vec::with_capacity(count),
            burst_damage: Vec::with_capacity(count),
            sustained_damage: Vec::with_capacity(count),
            shot_sorting: Vec::with_capacity(count),
            burst_sorting: Vec::with_capacity(count),
            sustained_sorting: Vec::with_capacity(count)
        };
        println!("Summing mods... ({:?}s elapsed)", start.elapsed());
        report_ag.calculate_modded_sums();
        println!("Calculating stats... ({:?}s elapsed)", start.elapsed());
        report_ag.calculate_modded_stats();
        println!("Calculating damage... ({:?}s elapsed)", start.elapsed());
        report_ag.calculate_damage();
        println!("Populating sorting tables... ({:?}s elapsed)", start.elapsed());
        report_ag.populate_sorting();
        println!("Sorting... ({:?}s elapsed)", start.elapsed());
        report_ag.sort();
        let duration = start.elapsed();
        println!("All done! Elapsed: {:?}\n", duration);
        report_ag
    }
    
    const HEADER: &'static str = "Hit|Burst|Sustain";
    
    pub fn display(&self, damage_criteria: DamageCriteria, results: usize, page: usize) {
        let sorting = match damage_criteria {
            DamageCriteria::PerShot => &self.shot_sorting,
            DamageCriteria::BurstDPS => &self.burst_sorting,
            DamageCriteria::SustainedDPS => &self.sustained_sorting
        };
        let p_min = results * page;
        let p_max = p_min + results;
        let mut buffer = String::with_capacity(250 * results);
        _= write!(
            buffer,
            "{}, sorted by {}\n{}\n",
            self.gun_data.name,
            damage_criteria.str(),
            Self::HEADER
        );
        for i in p_min..p_max {
            let id = sorting[i].combination_index as usize;
            self.add_report_row(&mut buffer, id);
        };
        println!("{}", buffer);
    }
    
    fn add_report_row(&self, buffer: &mut String, index_id: usize) {
        let shot = self.shot_damage[index_id];
        let burst = self.burst_damage[index_id];
        let sustained = self.sustained_damage[index_id];
        _ = write!(
            buffer,
            "{}|{}|{}\n",
            shot, burst, sustained
        );
        let mask = self.combinations[index_id];
        let combo = bits_to_mod_combo(mask);
        _ = write!(buffer, "Arcane: {}\n", self.loaded_mods.get_mod_name_u8(combo[8]));
        for mod_id in 0..8 {
            _ = write!(buffer, "{}, ", self.loaded_mods.get_mod_name_usize(mod_id));
        };
        buffer.pop();
        buffer.pop();
        _ = write!(buffer, "\n\n");
    }
    
    fn calculate_modded_sums(&mut self) {
        for bits in self.combinations.iter() {
            let combo = bits_to_mod_combo(bits.clone());
            let mod_sum = GunModSums::from_mod_list(&combo, &self.loaded_mods);
            self.modded_sums.push(mod_sum);
        };
    }
    
    fn calculate_modded_stats(&mut self) {
        for mod_sum in &self.modded_sums {
            self.modded_stats.push(self.gun_data.gun_stats.apply_stat_sums(mod_sum));
        };
    }
    
    fn calculate_damage(&mut self) {
        for stats in &self.modded_stats {
            let shot_damage = stats.calculate_shot_damage();
            let burst_damage = stats.calculate_burst_dps(shot_damage);
            let sustained_damage = stats.calculate_sustained_dps(burst_damage);
            self.shot_damage.push(shot_damage);
            self.burst_damage.push(burst_damage);
            self.sustained_damage.push(sustained_damage);
        };
    }
    
    fn populate_sorting(&mut self) {
        for (i, &damage) in self.shot_damage.iter().enumerate() {
            self.shot_sorting.push(SortingHelper::new(damage, i));
        };
        for (i, &damage) in self.burst_damage.iter().enumerate() {
            self.burst_sorting.push(SortingHelper::new(damage, i));
        };
        for (i, &damage) in self.sustained_damage.iter().enumerate() {
            self.sustained_sorting.push(SortingHelper::new(damage, i));
        };
    }
    
    fn sort(&mut self) {
        self.shot_sorting.sort_by_key(|r|r.sorting_integer);
        self.burst_sorting.sort_by_key(|r|r.sorting_integer);
        self.sustained_sorting.sort_by_key(|r|r.sorting_integer);
    }
    
}

fn bits_to_mod_combo(mut bits: u64) -> [u8; 9] {
    let mut mod_combo = [0; 9];
    let mut count = 0usize;
    while bits != 0 {
        let tz = bits.trailing_zeros() as u8;
        mod_combo[count] = tz;
        count += 1;
        bits &= bits -1;
    };
    mod_combo
}
