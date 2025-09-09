// use std::time::Instant;

use crate::workflows::cli_workflow_entry;

mod data;
mod combinatorics;
mod cli_inputs;
mod mod_parsing;
mod weapon_select;
mod context_core;
mod build_calc;
mod display;
mod workflows;

fn main() {
    cli_workflow_entry()
}