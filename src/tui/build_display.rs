use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use ratatui::crossterm::event::MouseButton;
use ratatui::layout::Constraint::{Fill, Min};
use ratatui::prelude::Constraint::{Length, Percentage};
use thousands::Separable;
use crate::build_calc::{calculate_builds, get_damage, GunModSums};
use crate::context_core::ModdingContext;
use crate::mod_parsing::LoadedMods;
use crate::tui::build_organization_structs::{BucketManager, BuildShowcase};
use crate::weapon_select::GunData;

const EXIT_KEYS: [KeyCode; 2] = [KeyCode::Backspace, KeyCode::Esc];
const BACK_TEXT: &str = "<== Go Back";

pub fn build_display_tui(
    terminal: &mut DefaultTerminal,
    gun_data: &GunData,
    modding_context: ModdingContext,
    base_sums: Option<GunModSums>
) {
    let loaded_mods = LoadedMods::new(&modding_context);
    let arcane_names = loaded_mods.get_arcane_names();
    let showcase = calculate_builds(
        &loaded_mods, &gun_data.gun_stats, &modding_context, None
    );
    let mut app = BuildDisplayApp::new(&showcase, &loaded_mods, gun_data, &modding_context);
    // let mut app = BuildDisplayApp::new(BuildShowcase::from_manager(&BucketManager::new(1)));
    terminal.draw(|frame| app.draw(frame)).unwrap();
    while app.running {
        let event = event::read();
        if let Ok(event) = event {
            match event {
                Event::Mouse(mouse_event) => {
                    app.handle_mouse_event(mouse_event);
                },
                Event::Key(key_event) => {
                    if EXIT_KEYS.contains(&key_event.code) {
                        app.running = false;
                    }
                },
                Event::Resize(_, _) => {
                    app.redraw = true;
                },
                _ => {}
            }
        }
    }
}


struct BuildDisplayApp<'a> {
    mouse_row: u16,
    mouse_column: u16,
    top_selection: u16,
    build_selection: u16,
    top_clicked: bool,
    build_clicked: bool,
    showcase: &'a BuildShowcase,
    loaded_mods: &'a LoadedMods,
    gun_data: &'a GunData,
    modding_context: &'a ModdingContext,
    running: bool,
    redraw: bool
} impl<'a> BuildDisplayApp<'a> {

    fn draw(&self, frame: &mut Frame) {
        // "best builds" refers to the per-arcane selection
        // "builds" refers to the top-8 builds for a given arcane
        // "mods" refers to the mods used in a given build
        // outer refers to the area a box will be drawn in
        // inner refers to the area inside a box
        // area refers to the space content will go
        // except top and bottom area since those are simple regions without whole naming schemes
        let vertical_layout = Layout::vertical([
            Length(3),
            Fill(1)
        ]);
        let [top_area, bottom_area] = vertical_layout.areas(frame.area());
        self.draw_help(frame, top_area);
        let bottom_inner = self.draw_lower_box(frame, bottom_area);
        let inner_layout = Layout::horizontal([
            Percentage(30),
            Fill(1)
        ]);
        let [best_builds_area, builds_outer_area] = inner_layout.areas(bottom_inner);
        self.draw_best_inner(frame, best_builds_area);
        let builds_inner = self.draw_builds_box(frame, builds_outer_area);
        let final_layout = Layout::horizontal([
            Percentage(50),
            Fill(1)
        ]);
        let [builds_area, mods_outer] = final_layout.areas(builds_inner);
        self.draw_builds_inner(frame, builds_area);
        let mods_area = self.draw_mods_outer(frame, mods_outer);
        self.draw_mods_inner(frame, mods_area);
    }

    // draws lower-box, returns inner area
    fn draw_lower_box(&self, frame: &mut Frame, bottom_area: Rect) -> Rect {
        let bottom_block = Block::bordered().title("Top Arcanes");
        let bottom_inner = bottom_block.inner(bottom_area);
        frame.render_widget(bottom_block, bottom_area);
        bottom_inner
    }

    // draws builds box, returns inner area
    fn draw_builds_box(&self, frame: &mut Frame, builds_area: Rect) -> Rect {
        let title_text = format!("{} Builds", self.get_selected_arcane_name());
        let builds_block = Block::bordered().title(title_text);
        let builds_inner = builds_block.inner(builds_area);
        frame.render_widget(builds_block, builds_area);
        builds_inner
    }

    // draws mods box, returns inner area
    fn draw_mods_outer(&self, frame: &mut Frame, composition_area: Rect) -> Rect {
        let title_text = format!("{} - #{}",self.get_selected_arcane_name(),  self.build_selection+1);
        let composition_block = Block::bordered().title(title_text);
        let composition_inner = composition_block.inner(composition_area);
        frame.render_widget(composition_block, composition_area);
        composition_inner

    }

    fn draw_best_inner(&self, frame: &mut Frame, area: Rect) {
        let arcane_names = self.loaded_mods.get_arcane_names();
        let top_builds = self.showcase.get_top_builds();
        let mut list: Vec<ListItem> = Vec::with_capacity(self.showcase.len);
        for i in 0..self.showcase.len {
            let build = top_builds[i];
            let arcane_name = if build.get_reference() < 1 {
                "No Arcane"
            } else {
                arcane_names[build.get_reference()-1]
            };
            let build_damage = build.get_damage();
            let mut build_string = build_damage.separate_with_commas();
            build_string.push(' ');
            build_string.push_str(arcane_name);
            let style = if i == self.top_selection as usize {
                Style::default().reversed()
            } else {
                Style::default()
            };
            let content = Line::styled(build_string, style);
            list.push(ListItem::new(content));
        }
        frame.render_widget(List::new(list), area);
    }

    fn draw_builds_inner(&self, frame: &mut Frame, area: Rect) {
        let top_build = self.showcase.get_top_builds()[self.top_selection as usize];
        let builds = self.showcase.get_build_list(top_build.get_reference());
        let len = builds.len();
        let mut list: Vec<ListItem> = Vec::with_capacity(len);
        for i in 0..len {
            let build = builds[i];
            let build_string = build.get_damage().separate_with_commas();
            let style = if i == self.build_selection as usize {
                Style::default().reversed()
            } else {
                Style::default()
            };
            let content = Line::styled(build_string, style);
            list.push(ListItem::new(content));
        }
        frame.render_widget(List::new(list), area);
    }

    fn draw_mods_inner(&self, frame: &mut Frame, area: Rect) {
        let strings = self.compose_scores();
        let mut list: Vec<ListItem> = Vec::with_capacity(strings.len());
        for string in strings {
            list.push(ListItem::new(string));
        }
        frame.render_widget(List::new(list), area);
    }

    fn compose_scores(&self) -> Vec<String> {
        // get specific mods
        let top_build = self.showcase.get_top_builds()[self.top_selection as usize];
        let builds = self.showcase.get_build_list(top_build.get_reference());
        let selected_build = builds[self.build_selection as usize];
        let mod_comp = self.loaded_mods.mod_combinations[selected_build.get_reference()];
        let mut all_mod_ids = Vec::with_capacity(9);
        for mod_id in mod_comp {
            all_mod_ids.push(mod_id);
        }
        if top_build.get_reference() > 0 {
            all_mod_ids.push(top_build.get_reference() as u8 + self.loaded_mods.arcane_count - 1);
        }
        // compute stats
        let mut full_sums = GunModSums::new();
        full_sums.add_many_mods(&all_mod_ids, self.loaded_mods);
        let full_damage = get_damage(self.modding_context, &self.gun_data.gun_stats, &full_sums);
        let mut mod_scores: Vec<(u8, i16)> = Vec::with_capacity(9);
        for mod_id in all_mod_ids {
            let mut reduced_sums = full_sums;
            reduced_sums.remove_mod(mod_id, self.loaded_mods);
            let reduced_damage = get_damage(self.modding_context, &self.gun_data.gun_stats, &reduced_sums);
            let damage_factor = full_damage/reduced_damage;
            let score = ((damage_factor - 1.0) * 1000.0).round() as i16;
            mod_scores.push((mod_id, score));
        }
        mod_scores.sort_by_key(|pair|-pair.1);
        // turn to string
        let mut strings: Vec<String> = Vec::with_capacity(9);
        for (mod_id, score) in mod_scores {
            let mut string = String::with_capacity(40);
            let number_string = score.separate_with_commas();
            string.push_str(self.loaded_mods.get_name(mod_id));
            string.push_str(" (");
            string.push_str(&number_string);
            string.push(')');
            strings.push(string);
        }
        strings
    }

    fn get_selected_arcane_name(&self) -> &str {
        let arcanes = self.loaded_mods.get_arcane_names();
        let arcane_id = self.showcase.get_top_builds()[self.top_selection as usize].get_reference();
        if arcane_id == 0 {
            "No Arcane"
        } else {
            arcanes[arcane_id - 1]
        }
    }

    // draws upper area
    fn draw_help(&self, frame: &mut Frame, top_area: Rect) {
        let top_layout = Layout::horizontal([
            Fill(1),
            Length(BACK_TEXT.len() as u16 + 2)
        ]);
        let [help_area, button_area] = top_layout.areas(top_area);
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        self.mouse_row = mouse_event.row;
        self.mouse_column = mouse_event.column;
    }

    fn new(
        showcase: &'a BuildShowcase, loaded_mods: &'a LoadedMods, gun_data: &'a GunData, modding_context: &'a ModdingContext
    ) -> Self {
        Self {
            mouse_row: 0,
            mouse_column: 0,
            top_selection: 0,
            build_selection: 0,
            top_clicked: false,
            build_clicked: false,
            showcase,
            loaded_mods,
            gun_data,
            modding_context,
            running: true,
            redraw: false
        }
    }

}