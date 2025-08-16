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

mod data;
mod gun_structs;
mod combinatorics;

fn main() {
    println!("hello world");

    let c = 60;
    let mut old_time = Duration::default();
    let mut start = Instant::now();

    let old_combos = combinatorics::generate_combinations(c);
    old_time = start.elapsed();

    println!("All done! {:?} elapsed.", old_time);
    println!("old_first, old_last, {:?}, {:?}", &old_combos.first(), &old_combos.last());
}