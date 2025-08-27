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
use crate::weapon_select::{weapon_select, GunData};

mod data;
mod combinatorics;
mod cli_inputs;
mod mod_parsing;
mod weapon_select;
mod context_core;
mod build_calc;

fn main() {
    weapon_select();
}