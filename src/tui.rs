use ratatui::crossterm::event::{MouseButton, MouseEventKind};

pub mod weapon_search_menu;
pub mod context_menu;

mod stat_screen;
pub mod build_organization_structs;
mod build_display;

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