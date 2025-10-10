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
    // struct_sizing();
    workflows::cli_workflow_entry();
}