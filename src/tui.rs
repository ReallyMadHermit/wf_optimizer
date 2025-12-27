use std::io::stdout;
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture, MouseButton, MouseEventKind};
use ratatui::crossterm::terminal;
use ratatui::crossterm::ExecutableCommand;

pub mod weapon_search_menu;
pub mod context_menu;

mod stat_screen;
pub mod build_organization_structs;
mod build_display;

pub fn main() {
    _ = stdout().execute(SetCursorStyle::BlinkingUnderScore);
    _ = stdout().execute(EnableMouseCapture);
    _ = terminal::enable_raw_mode();

    let mut terminal = ratatui::init();
    let gun_data = weapon_search_menu::weapon_search_tui(&mut terminal, None);
    context_menu::context_menu_tui(&mut terminal, gun_data);

    ratatui::restore();
    _ = stdout().execute(DisableMouseCapture);
    _ = terminal::disable_raw_mode();
}

fn clicked(mouse_event_kind: MouseEventKind) -> i8 {
    if let MouseEventKind::Down(button) = mouse_event_kind {
        if button == MouseButton::Left {
            1
        } else {
            -1
        }
    } else {
        0
    }
}