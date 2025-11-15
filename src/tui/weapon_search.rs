use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use ratatui::crossterm::event::MouseButton;
use crate::weapon_select::GunData;

const BUFFER_LENGTH: usize = 15;  // for the input String the user types into
const SELECTION_START: u16 = 5;  // for what row that results start on in the search

pub fn weapon_search_tui(terminal: &mut DefaultTerminal, current_selection: Option<GunData>) -> Option<GunData> {
    let mut app = WeaponSearchApp::new(current_selection);
    terminal.draw(|frame| app.draw(frame)).unwrap();
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
            terminal.draw(|frame| app.draw(frame)).unwrap();
            app.redraw = false;
        }
    }
    app.returning
}

struct WeaponSearchApp {
    weapon_names_vec: Vec<(&'static str, &'static str)>,
    string_buffer: String,
    results: Vec<u16>,
    display: u16,
    cursor: u16,
    mouse: (u16, u16),  // TODO: "row" is used as x and "column" is never used lmao, that's silly
    redraw: bool,
    running: bool,
    returning: Option<GunData>
} impl WeaponSearchApp {

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }
        match key_event.code {
            KeyCode::Char(char) => self.enter_char(char),
            KeyCode::Backspace => self.back_space(),
            // KeyCode::Esc => {
            //     self.running = false;
            // }
            _=> {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let pos = (mouse_event.row, mouse_event.column);
        if pos != self.mouse {
            self.mouse = pos;
            self.redraw = true;
        }
        let clicked = matches!(mouse_event.kind, MouseEventKind::Down(_));
        if let MouseEventKind::Down(button) = mouse_event.kind {
            self.handle_click(button);
        }
        // if clicked && self.clicked_result().is_some() {
        //     self.handle_click();
        // }
    }

    fn handle_click(&mut self, button: MouseButton) {
        if button == MouseButton::Left {
            let adjusted_index = self.mouse.0 - SELECTION_START;
            let weapon_index = self.results[adjusted_index as usize];
            self.returning = Some(GunData::from_index(weapon_index as usize));
            // self.returning = Some(self.weapon_names()[weapon_index as usize].0);
        }
        self.running = false;
    }

    fn draw(&mut self, frame: &mut Frame) {
        // layout
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1)
        ]);
        let [help_area, input_area, message_area] = vertical.areas(frame.area());
        self.display = message_area.height.clamp(0, self.results.len() as u16);
        self.draw_instruction(frame, help_area);
        self.draw_input(frame, input_area);

        // results
        self.draw_results(frame, message_area);
    }

    fn draw_instruction(&self, frame: &mut Frame, area: Rect) {
        let msg = vec![
            "Select your weapon by entering a ".into(),
            "weapon name".bold(),
            " and ".into(),
            "clicking".bold(),
            " the desired result. ".into(),
            "Right Click".bold(),
            " to go back.".into()
        ];
        let text = Text::from(Line::from(msg)).patch_style(Style::default());
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, area);
    }

    fn draw_input(&self, frame: &mut Frame, area: Rect) {
        let input = Paragraph::new(self.string_buffer.as_str()).style(Style::default())
            .block(Block::bordered().title("Search"));
        frame.render_widget(input, area);
        frame.set_cursor_position(Position::new(area.x + self.cursor + 1, area.y + 1));
    }

    fn draw_results(&self, frame: &mut Frame, area: Rect) {
        let in_results = self.mouse.0 >= SELECTION_START;
        let mut line_items: Vec<ListItem> = Vec::with_capacity(self.display as usize);
        for i in 0..self.display {
            let name_index = self.results[i as usize];
            let (name, attack) = self.weapon_names()[name_index as usize];

            let style = if in_results && i + SELECTION_START == self.mouse.0 {
                Style::default().reversed()
            } else {
                Style::default()
            };

            let content = Line::from(Span::styled(format!("{name}; {attack}"), style));
            let list_item = ListItem::new(content);
            line_items.push(list_item);
        };
        let r_len = self.results.len();
        let d = self.display;
        let list = List::new(line_items).block(Block::bordered().title(format!(
            "Results ({d}/{r_len})"
        )));
        frame.render_widget(list, area);
    }

    fn new(current_selection: Option<GunData>) -> Self {
        let weapon_names_vec = get_weapon_names();
        let cap = weapon_names_vec.len();
        Self {
            weapon_names_vec,
            string_buffer: String::with_capacity(BUFFER_LENGTH),
            results: Vec::from_iter(0..(cap-1) as u16),
            display: 0,
            cursor: 0,
            mouse: (0, 0),
            redraw: true,
            running: true,
            returning: current_selection
        }
    }

    fn weapon_names(&self) -> &[(&'static str, &'static str)] {
        &self.weapon_names_vec[1..]
    }

    fn enter_char(&mut self, new_char: char) {
        self.string_buffer.push(new_char);
        self.retain_results();
        self.redraw = true;
        self.cursor += 1;
    }

    fn back_space(&mut self) {
        let cap: usize = self.results.capacity();
        let len = self.string_buffer.len();
        self.results = Vec::from_iter(0..cap as u16);
        if len > 1 {
            self.string_buffer.pop();
            self.retain_results();
            self.cursor -=1;
        } else {
            self.string_buffer.clear();
            self.cursor = 0;
        };
        self.redraw = true;
    }

    fn retain_results(&mut self) {
        let input = self.string_buffer.to_ascii_lowercase();
        self.results.retain_mut(
            |i|
                self.weapon_names_vec[(*i+1) as usize].0
                    .to_ascii_lowercase()
                    .contains(&input)
        );
    }

    fn clicked_result(&self) -> Option<u16> {
        if self.mouse.0 >= SELECTION_START && (self.mouse.0 - SELECTION_START) < self.display {
            Some(self.mouse.0 - SELECTION_START)
        } else {
            None
        }
    }

}

fn get_weapon_names() -> Vec<(&'static str, &'static str)> {
    let lines = Vec::from_iter(crate::data::GUN_DATA.lines());
    let mut names = Vec::with_capacity(lines.len());
    for line in lines {
        let split = Vec::from_iter(line.split(","));
        names.push((split[1], split[2]));
    };
    names
}