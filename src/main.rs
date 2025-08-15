// mod mod_structs;
// mod weapon_structs;
// mod brute_force_solution;
// mod file_interfacing;
// mod cli_inputs;
// mod gun_core;
pub mod old;

use old::cli_inputs::cli_build_calculation_workflow;

const TOP_BUILD_COUNT: usize = 20;

fn main() {
    cli_build_calculation_workflow();
}