// use std::time::Instant;
use std::mem::size_of;

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
    // println!("{:?}", size_of::<Probable>());
    struct_sizing();
    cli_workflow_entry()
}

fn struct_sizing() {
    println!("Byte-Sizes");
    println!("LoadedMods: {:?}", size_of::<mod_parsing::LoadedMods>());
    println!("ModdingContext: {:?}", size_of::<context_core::ModdingContext>());
}