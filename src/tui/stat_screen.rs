use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use ratatui::crossterm::event::MouseButton;
use ratatui::layout::Constraint::Min;
use ratatui::prelude::Constraint::Length;
use crate::mod_parsing::ModStatType;

const VALUE_LENGTH: usize = 8;
const OPTIONS_OFFSET: u16 = 2;
const COLUMN_START: u16 = 2;
const DISPLAY_CAPACITY: usize = 128;

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
                    app.handle_key_event(key_event);
                },
                Event::Mouse(mouse_event) => {
                    app.handle_mouse_event(mouse_event);
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

    if app.stat_fields.has_values() {
        Some(app.stat_fields)
    } else {
        None
    }
}


struct StatScreenApp {
    stat_fields: StatFields,
    buffer: String,
    hovered_row: u16,
    selected_row: Option<u16>,
    running: bool,
    redraw: bool,
    riven: bool,
    negative_input: bool,  // if the value in the buffer is a negative number (we don't store '-')
    highlight_selection: bool  // if over-writing an existing field value
} impl StatScreenApp {

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }
        self.redraw = true;
        match key_event.code {
            KeyCode::Char(char) => {
                match char {
                    '0'..='9' => {
                        self.clear_test();
                        self.buffer.push(char);
                    },
                    '.' => {
                        if !self.buffer.is_empty() {
                            self.buffer.push(char);
                        }
                    }
                    '-' => {
                        if !self.negative_input {
                            self.clear_test();
                            self.negative_input = true;
                        }
                    },
                    '+' => {
                        if self.negative_input {
                            self.clear_test();
                            self.negative_input = false;
                        }
                    },
                    _ => {}
                }
            },
            KeyCode::Backspace | KeyCode::Delete => {
                if self.selected_row.is_none() {
                    self.running = false;
                } else if self.buffer.is_empty() {
                    self.negative_input = false;
                } else {
                    self.clear_test();
                    self.buffer.pop();
                }
            },
            KeyCode::Enter => {
                if self.selected_row.is_some() {
                    self.push_buffer();
                } else {
                    self.running = false;
                }
            }
            KeyCode::Tab => {
                if let Some(row) = self.selected_row {
                    self.push_buffer();
                    if self.stat_fields.contains(row + 1) {
                        self.left_click(row + 1);
                        self.hovered_row = row + 3;
                    } else {
                        self.left_click(0);
                        self.hovered_row = 2;
                    }
                } else {
                    self.left_click(0);
                    self.hovered_row = 2;
                }
            },
            KeyCode::Esc => {
                self.running = false;
            },
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let row = mouse_event.row;
        if row != self.hovered_row {
            self.hovered_row = row;
            self.redraw = true;
        }
        let clicked: i8 = if let MouseEventKind::Down(button) = mouse_event.kind {
            if button == MouseButton::Left {
                1
            } else {
                -1
            }
        } else {
            0
        };
        if clicked != 0 {
            self.click(clicked > 0);
            self.redraw = true;
        }
    }

    fn click(&mut self, left: bool) {
        if self.hovered_row >= 2 && self.stat_fields.contains(self.hovered_row - 2) {
            let field_id = self.hovered_row - 2;
            self.push_buffer();
            if left {
                self.left_click(field_id);
            } else {
                self.right_click(field_id);
            }
        }
    }

    fn left_click(&mut self, field_id: u16) {
        let (stat, value) = self.stat_fields.get(field_id).unwrap_or_default();
        if stat != ModStatType::None {
            self.selected_row = Some(field_id);
        } else {
            return;
        }
        if value < 0 {
            self.negative_input = true;
        }
        if value != 0 {
            self.highlight_selection = true;
            match stat {
                ModStatType::FinalCritDamage => {
                    self.buffer += &(value as f32 / 100.0).abs().to_string();
                },
                _ => {
                    self.buffer += &value.abs().to_string();
                }
            }
        } else {
            self.highlight_selection = false;
        }
    }

    fn right_click(&mut self, field_id: u16) {
        if let Some(row) = self.selected_row {
            if field_id == row {
                self.selected_row = None;
                self.buffer.clear();
            }
        }
        let (stat, value) = self.stat_fields.get(field_id).unwrap_or_default();
        if stat != ModStatType::None {
            self.stat_fields.fields[field_id as usize] = Some((stat, 0));
        }
    }

    fn clear_test(&mut self) {
        if self.highlight_selection {
            self.buffer.clear();
            self.negative_input = false;
            self.highlight_selection = false;
        }
    }

    fn push_buffer(&mut self) {
        let row = if let Some(row) = self.selected_row {
            row
        } else {
            self.buffer.clear();
            self.negative_input = false;
            self.selected_row = None;
            return;
        };

        let stat_field = self.stat_fields.get(row).unwrap_or_default().0;
        let input_number = if let Ok(f) = self.buffer.parse::<f32>() {
            if self.negative_input {
                -f
            } else {
                f
            }
        } else if self.buffer.is_empty() {
            0.0
        } else {
            return;
        };
        self.buffer.clear();
        self.selected_row = None;
        self.negative_input = false;

        match stat_field {
            ModStatType::None => {},
            ModStatType::FinalCritDamage => {
                self.stat_fields.fields[row as usize] = Some((stat_field, (input_number * 100.0).round() as i16));
            },
            _ => {
                self.stat_fields.fields[row as usize] = Some((stat_field, input_number.round() as i16));
            }
        }
    }

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
        let stat_fields = self.stat_fields.get_all();
        let mut fields: Vec<ListItem> = Vec::with_capacity(stat_fields.len());
        for (i, option) in stat_fields.iter().enumerate() {
            let (stat, value) = option.unwrap_or_default();
            let selected = self.selected_row == Some(i as u16);
            let row_string = self.write_row_string(stat, value, selected);
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
        if let Some(i) = self.selected_row {
            let row = OPTIONS_OFFSET + i;
            let column = COLUMN_START + self.buffer.len() as u16;
            if self.highlight_selection {
                let (start, width) = if self.negative_input {
                    (COLUMN_START - 1, self.buffer.len() as u16 + 1)
                } else {
                    (COLUMN_START, self.buffer.len() as u16)
                };
                let rect = Rect::new(start, row, width, 1);
                frame.render_widget(Block::default().style(Style::default().reversed()), rect);
            } else {
                frame.set_cursor_position(Position::new(column, row));
            }

        }
    }

    fn get_row_style(&self, field_number: usize) -> Style {
        if self.hovered_row >= OPTIONS_OFFSET {
            let field_id = (self.hovered_row - OPTIONS_OFFSET) as usize;
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
            if self.negative_input {
                string_buffer.push('-');
            } else {
                string_buffer.push('+');
            }
            string_buffer += &self.buffer;
            if !self.highlight_selection {
                string_buffer.push(' ');
            }
        } else {
            if stat_value < 0 {
                string_buffer.push('-');
            } else {
                string_buffer.push('+');
            }
            string_buffer += &value_string;
        }
        string_buffer += name_prefix;
        while string_buffer.len() < VALUE_LENGTH {
            string_buffer.push(' ');
        }
        string_buffer += field_name;
        string_buffer
    }

    fn edit_buffs(stat_fields: StatFields, riven: bool) -> Self {
        Self {
            stat_fields,
            buffer: String::with_capacity(10),
            hovered_row: 0,
            selected_row: None,
            running: true,
            redraw: true,
            riven,
            negative_input: false,
            highlight_selection: false
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
            selected_row: None,
            running: true,
            redraw: false,
            riven,
            negative_input: false,
            highlight_selection: false
        }
    }

}


pub struct StatFields {
    fields: [Option<(ModStatType, i16)>; 12],
    len: u8,
} impl StatFields {

    fn default() -> Self {
        Self {
            fields: [None; 12],
            len: 0,
        }
    }

    fn push(&mut self, stat_type: ModStatType, value: i16) {
        self.fields[self.len as usize] = Some((stat_type, value));
        self.len +=1;
    }

    fn get_all(&self) -> &[Option<(ModStatType, i16)>] {
        &self.fields[0..self.len as usize]
    }

    fn get(&self, field_id: u16) -> Option<(ModStatType, i16)> {
        if self.contains(field_id) {
            self.fields[field_id as usize]
        } else {
            None
        }
    }

    fn contains(&self, field_id: u16) -> bool {
        (0..self.len as u16).contains(&field_id)
    }

    fn has_values(&self) -> bool {
        for option in self.get_all() {
            let (stat, value) = option.unwrap_or_default();
            if value != 0 {
                return true;
            }
        }
        false
    }

    pub fn display(&self) -> String {
        let mut string = String::with_capacity(DISPLAY_CAPACITY);
        for option in self.get_all() {
            let (stat, value) = option.unwrap_or_default();
            if value == 0 {
                continue;
            } else if value > 0 {
                string.push('+');
            } else {
                string.push('-');
            }
            match stat {
                ModStatType::None => {},
                ModStatType::FinalCritDamage => {
                    string += &((value.abs() as f32) / 100.0 ).to_string();
                },
                _ => {
                    string += &value.abs().to_string();
                }
            }
            string += stat.stat_prefix();
            string += stat.to_str();
            string += ", ";
        }
        string.pop();
        string.pop();
        string
    }

}
