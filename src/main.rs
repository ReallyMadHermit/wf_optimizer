// mod mod_structs;
// mod weapon_structs;
// mod brute_force_solution;
// mod file_interfacing;
// mod cli_inputs;
// mod gun_core;

// pub mod old;
//
// use old::cli_inputs::cli_build_calculation_workflow;
//
// const TOP_BUILD_COUNT: usize = 20;

use std::time::{Duration, Instant};
use mod_parsing::load_mods;

mod data;
mod structs;
mod combinatorics;
mod cli_inputs;
mod traits;
mod impl_blocks;
mod mod_parsing;
mod weapon_select;
mod context_core;
mod build_calc;

fn main() {
    // let (gun_data, modding_context) = establish_the_facts();
    let loaded_mods = load_mods(&modding_context);
    for name in loaded_mods.mod_names {
        println!("{}", name);
    }
}