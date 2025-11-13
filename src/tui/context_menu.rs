use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use ratatui::layout::Constraint::{Fill, Length, Percentage, Min};
use crate::context_core::{DamageCriteria, ModdingContext};
use crate::weapon_select::GunData;


const NUMBER_BUFFER_LENGTH: usize = 6;
const DISPLAY_STRING_LENGTH: usize = 64;
const LABEL_LENGTH: usize = 20;
const OPTIONS_OFFSET: usize = 2;


pub fn context_menu_tui() {
    let mut app = ContextMenuApp::new();
    let mut terminal = ratatui::init();
    while app.running {
        if app.redraw {
            _ = terminal.draw(|frame| app.draw(frame));
            app.redraw = false;
        }
    }
    ratatui::restore();
}


struct ContextMenuApp {
    weapon_selection: Option<GunData>,
    damage_criteria: DamageCriteria,
    has_kills: bool,
    ads: bool,
    acuity: bool,
    amalgam: bool,
    bane: u8,
    status_count: u8,
    running: bool,
    redraw: bool
} impl ContextMenuApp {

    fn new() -> Self {
        Self {
            weapon_selection: None,
            damage_criteria: DamageCriteria::default(),
            has_kills: true,
            ads: true,
            acuity: false,
            amalgam: false,
            bane: 0,
            status_count: 0,
            running: true,
            redraw: true
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical_layout = Layout::vertical([
            Min(2),
            Length(11),
            Length(3)
        ]).split(frame.area());
        let buttons_layout = Layout::horizontal([
            Fill(1),
            Percentage(20)
        ]).split(vertical_layout[1]);
        self.draw_instructions(frame, vertical_layout[0]);
    }

    fn draw_instructions(&mut self, frame: &mut Frame, area: Rect) {
        let top_line = Line::from("Configure your build criteria below.");
        let bottom_text = vec![
            "Left Click".bold(),
            " increments or toggles, ".into(),
            "Right Click".bold(),
            " decrements or resets. ".into(),
            "Esc".bold(),
            " quits.".into()
        ];
        let bottom_line = Line::from(bottom_text).patch_style(Style::default());
        let list = List::new(
            [
                top_line,
                bottom_line
            ]
        ).style(Style::default());
        frame.render_widget(list, area);
    }

}


#[derive(Clone, Copy, Eq, PartialEq)]
enum FieldType {
    SelectedWeapon,
    DamageCriteria,
    HasKills,
    ADS,
    Acuity,
    Amalgam,
    BaneMods,
    StatusCount,
    AppliedBuffs,
    RivenStats,
    FunkyNumbers
} impl FieldType {

    const COUNT: usize = 11;
    const MAX: usize = Self::COUNT + OPTIONS_OFFSET;
    const FIELDS: [Self; Self::COUNT] = [
        Self::SelectedWeapon,
        Self::DamageCriteria,
        Self::HasKills,
        Self::ADS,
        Self::Acuity,
        Self::Amalgam,
        Self::BaneMods,
        Self::StatusCount,
        Self::AppliedBuffs,
        Self::RivenStats,
        Self::FunkyNumbers
    ];

    fn get_type(row: u16) -> Option<Self> {
        let n = row as usize;
        if (OPTIONS_OFFSET..Self::MAX).contains(&n) {
            Some(Self::FIELDS[n-OPTIONS_OFFSET])
        } else {
            None
        }
    }

    fn get_label(&self) -> String {
        let base_text = match self {
            Self::SelectedWeapon => "Selected Weapon",
            Self::DamageCriteria => "Damage Criteria",
            Self::HasKills => "Has Kills",
            Self::ADS => "Aims Down Sights",
            Self::Acuity => "Use Acuity",
            Self::Amalgam => "Use Amalgam Mod",
            Self::BaneMods => "Use Bane Mod",
            Self::StatusCount => "Status Effects",
            Self::AppliedBuffs => "Applied Buffs",
            Self::RivenStats => "Riven Mod",
            Self::FunkyNumbers => "fUmkYdbUnknUmbgErs"
        };
        let mut buffer = String::with_capacity(DISPLAY_STRING_LENGTH);
        buffer.push_str(base_text);
        while buffer.len() < LABEL_LENGTH {
            buffer.push('.');
        }
        buffer.push_str(": ");
        buffer
    }

}


fn field_label(field_name: &str) -> String {
    let mut buffer = String::with_capacity(DISPLAY_STRING_LENGTH);
    buffer.push_str(field_name);
    while buffer.len() < LABEL_LENGTH {
        buffer.push('.');
    };
    buffer.push_str(": ");
    buffer
}


fn display<'a>(
    field_name: &'static str, field_contents: &'static str, hovered: bool
) -> ListItem<'a> {
    let mut buffer = field_label(field_name);
    buffer.push_str(field_contents);
    let style = if hovered {
        Style::default().reversed()
    } else {
        Style::default()
    };
    ListItem::new(Line::from(Span::styled(buffer, style)))
}
