use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use crate::context_core::ModdingContext;
use crate::weapon_select::GunData;

struct ContextMenuApp {
    weapon_selection: Option<GunData>,
    context_core: Option<ModdingContext>
}