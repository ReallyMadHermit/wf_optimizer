use std::io::stdout;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use ratatui::crossterm::cursor::SetCursorStyle;
use ratatui::crossterm::event::KeyEvent;
use ratatui::crossterm::ExecutableCommand;
use ratatui::layout::Rect;

const BUFFER_LENGTH: usize = 15;

pub fn weapon_search_tui() -> Option<u16> {
    _ = stdout().execute(SetCursorStyle::BlinkingUnderScore);
    let mut app = WeaponSearchApp::new();
    let mut terminal = ratatui::init();
    while app.running {
        if app.redraw {
            terminal.draw(|frame| app.draw(frame)).unwrap();
            app.redraw = false;
        }
        let event = event::read();
        if let Ok(Event::Key(key_event)) = event {
            app.handle_key_event(key_event);
        } else if let Ok(Event::Mouse(mouse)) = event {

        } else if let Ok(Event::Resize(w, h)) = event {
            app.redraw = true;
        }
    };
    ratatui::restore();
    app.returning
}

struct WeaponSearchApp {
    weapon_names_vec: Vec<(&'static str, &'static str)>,
    string_buffer: String,
    results: Vec<u16>,
    scroll: u16,
    display: u16,
    cursor: u16,
    redraw: bool,
    running: bool,
    returning: Option<u16>
} impl WeaponSearchApp {

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.kind != KeyEventKind::Press {
            return;
        }
        match key_event.code {
            KeyCode::Char(char) => self.enter_char(char),
            KeyCode::Backspace => self.back_space(),
            KeyCode::Esc => {
                self.running = false;
            }
            _=> {}
        }
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
            "Esc".bold(),
            " to quit.".into()
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
        let mut line_items: Vec<ListItem> = Vec::with_capacity(self.display as usize);
        for i in 0..self.display {
            let name_index = self.results[(i + self.scroll) as usize];
            let (name, attack) = self.weapon_names()[name_index as usize];
            let content = Line::from(Span::raw(format!("{name}; {attack}")));
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

    fn new() -> Self {
        let weapon_names_vec = get_weapon_names();
        let cap = weapon_names_vec.len();
        Self {
            weapon_names_vec,
            string_buffer: String::with_capacity(BUFFER_LENGTH),
            results: Vec::from_iter(0..(cap-1) as u16),
            scroll: 0,
            display: 0,
            cursor: 0,
            redraw: true,
            running: true,
            returning: None
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
        self.scroll = 0;
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

    fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(crate::tui::SCROLL_AMOUNT);
    }

    fn scroll_down(&mut self) {
        let cap = (self.results.len() as u16).saturating_sub(self.display);
        self.scroll = self.scroll.saturating_add(crate::tui::SCROLL_AMOUNT).clamp(0, cap);
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