use std::io::stdout;
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::ExecutableCommand;
use ratatui::crossterm::terminal;

mod data;
mod combinatorics;
mod cli_inputs;
mod mod_parsing;
mod weapon_select;
mod context_core;
mod build_calc;
mod buff_effect;
mod tui;
mod legacy;
mod memory;

fn main() {
    _ = stdout().execute(SetCursorStyle::BlinkingUnderScore);
    _ = stdout().execute(EnableMouseCapture);
    _ = terminal::enable_raw_mode();
    let r = tui::weapon_search::weapon_search_tui(None);
    tui::context_menu::context_menu_tui();

    _ = stdout().execute(DisableMouseCapture);
    _ = terminal::disable_raw_mode();
    // if let Some(g) = r {
    //     println!("{}, {}", g.name, g.fire_mode);
    // }
}