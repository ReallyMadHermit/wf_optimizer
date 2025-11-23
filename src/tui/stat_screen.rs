use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use ratatui::crossterm::event::MouseButton;
use ratatui::layout::Constraint::Min;
use ratatui::prelude::Constraint::Length;
use crate::mod_parsing::ModStatType;

const BUFF_STATS: [ModStatType; 11] = [
    ModStatType::Damage,
    ModStatType::Elemental,
    ModStatType::FireRate,
    ModStatType::CritChance,
    ModStatType::CritDamage,
    ModStatType::FlatCritChance,
    ModStatType::FinalCritDamage,
    ModStatType::Bane,
    ModStatType::ReloadSpeed,
    ModStatType::Multishot,
    ModStatType::StatusChance
];

const RIVEN_STATS: [ModStatType; 12] = [
    ModStatType::Damage,
    ModStatType::Multishot,
    ModStatType::CritChance,
    ModStatType::CritDamage,
    ModStatType::Cold,
    ModStatType::Shock,
    ModStatType::Heat,
    ModStatType::Toxic,
    ModStatType::StatusChance,
    ModStatType::FireRate,
    ModStatType::MagazineCapacity,
    ModStatType::ReloadSpeed
];


pub fn stat_screen_tui(
    terminal: &mut DefaultTerminal,
    stat_fields: Option<StatFields>,
    riven: bool
) -> Option<StatFields> {
    let mut app = if let Some(stat_fields) = stat_fields {
        StatScreenApp::edit_buffs(stat_fields, riven)
    } else {
        StatScreenApp::new(riven)
    };

    _ = terminal.draw(|frame| app.draw(frame)).unwrap();
    while app.running {
        let event = event::read();
        if let Ok(event) = event {
            match event {
                Event::Key(key_event) => {
                    
                },
                Event::Mouse(mouse_event) => {
                    // app.handle_mouse_event(mouse_event);
                },
                Event::Resize(_, _) => {
                    app.redraw = true;
                },
                _ => {}
            }
        }
        if app.redraw {
            _ = terminal.draw(|frame| app.draw(frame)).unwrap();
            app.redraw = false;
        }
    }

    if app.stat_fields.has_values {
        Some(app.stat_fields)
    } else {
        None
    }
}


struct StatScreenApp {
    stat_fields: StatFields,
    buffer: String,
    hovered_row: i16,
    selected_field: Option<ModStatType>,
    running: bool,
    redraw: bool,
    riven: bool
} impl StatScreenApp {

    fn draw(&mut self, frame: &mut Frame) {
        let area_size = self.stat_fields.len + 2;
        let vertical_layout = Layout::vertical([
            Length(1),
            Min(area_size as u16),
            Length(3)
        ]);
        let [help_area, field_area, button_area] = vertical_layout.areas(frame.area());
        self.draw_instructions(frame, help_area);
        self.draw_fields(frame, field_area);
    }

    fn draw_instructions(&mut self, frame: &mut Frame, area: Rect) {
        let msg = vec![
            "Left Click".bold(),
            " a stat to edit its value, ".into(),
            "Right Click".bold(),
            " to clear its value.".into()
        ];
        let line = Line::from(msg).patch_style(Style::default());
        frame.render_widget(line, area);
    }

    fn draw_fields(&mut self, frame: &mut Frame, area: Rect) {
        let stat_fields = self.stat_fields.get();
        let mut fields: Vec<ListItem> = Vec::with_capacity(stat_fields.len());
        let selected_field = if let Some(field) = self.selected_field {
            field
        } else {
            ModStatType::None
        };
        for (i, option) in stat_fields.iter().enumerate() {
            let &(stat, value) = if let Some(pair) = option {
                pair
            } else {
                &(ModStatType::None, 0)
            };
            let row_string = self.write_row_string(stat, value, stat == selected_field);
            let content = Line::from(Span::styled(row_string, self.get_row_style(i)));
            let line = ListItem::new(content);
            fields.push(line);
        }
        let title = if self.riven {
            "Riven Stats"
        } else {
            "Buff Stats"
        };
        frame.render_widget(List::new(fields).block(Block::bordered().title(title)), area);
    }

    fn get_row_style(&self, field_number: usize) -> Style {
        if self.hovered_row >= 2 {
            let field_id = (self.hovered_row - 2) as usize;
            if field_id == field_number {
                return Style::default().reversed();
            }
        }
        Style::default()
    }

    fn write_row_string(
        &self, stat_type: ModStatType, stat_value: i16, selected: bool
    ) -> String {
        let field_name = stat_type.to_str();
        let name_prefix = stat_type.stat_prefix();
        let value_string = if stat_type == ModStatType::FinalCritDamage {
            ((stat_value.abs() as f32) / 100.0).to_string()
        } else {
            stat_value.abs().to_string()
        };

        let mut string_buffer = String::with_capacity(field_name.len() + 8);
        if selected {
            string_buffer += &self.buffer;
        } else if stat_value < 0 {
            string_buffer.push('-');
            string_buffer += &value_string;
        } else {
            string_buffer.push('+');
            string_buffer += &value_string;
        }
        string_buffer += name_prefix;
        string_buffer += field_name;
        string_buffer
    }

    fn edit_buffs(stat_fields: StatFields, riven: bool) -> Self {
        Self {
            stat_fields,
            buffer: String::with_capacity(10),
            hovered_row: 0,
            selected_field: None,
            running: true,
            redraw: true,
            riven
        }
    }

    fn new(riven: bool) -> Self {
        let mut stat_fields = StatFields::default();
        let stat_array: &[ModStatType] = if riven {
            &RIVEN_STATS
        } else {
            &BUFF_STATS
        };
        for &stat_type in stat_array {
            stat_fields.push(stat_type, 0);
        }
        Self {
            stat_fields,
            buffer: String::with_capacity(10),
            hovered_row: 0,
            selected_field: None,
            running: true,
            redraw: false,
            riven
        }
    }

}


pub struct StatFields {
    fields: [Option<(ModStatType, i16)>; 12],
    len: u8,
    has_values: bool
} impl StatFields {

    fn default() -> Self {
        Self {
            fields: [None; 12],
            len: 0,
            has_values: false
        }
    }

    fn push(&mut self, stat_type: ModStatType, value: i16) {
        self.fields[self.len as usize] = Some((stat_type, value));
        self.len +=1;
        if value != 0 {
            self.has_values = true;
        }
    }

    fn get(&self) -> &[Option<(ModStatType, i16)>] {
        &self.fields[0..self.len as usize]
    }

}
