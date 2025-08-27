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
mod combinatorics;
mod cli_inputs;
mod mod_parsing;
mod weapon_select;
mod context_core;
mod build_calc;

fn main() {
    for n in 0..5 {
        let i = cli_inputs::UserInput::looped_integer_prompt(
            "enta it",
            1,
            10,
            7
        );
        println!("{}", i);
    };
}