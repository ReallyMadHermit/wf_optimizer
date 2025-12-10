use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyEvent, MouseEvent, MouseEventKind}, layout::{Constraint, Layout, Position, Rect}, style::{Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, List, ListItem, Paragraph}, DefaultTerminal, Frame};
use ratatui::crossterm::event::MouseButton;
use ratatui::layout::Constraint::{Fill, Length, Percentage, Min};
use crate::build_calc::GunModSums;
use crate::context_core::{DamageCriteria, ModdingContext, WeaponType};
use crate::mod_parsing::{LoadedMods, ModData};
use crate::tui::stat_screen::StatFields;
use crate::weapon_select::GunData;
use crate::tui::weapon_search_menu::weapon_search_tui;
use crate::tui::stat_screen::stat_screen_tui;
use crate::tui::build_display::build_display_tui;
use super::clicked;

const DISPLAY_STRING_LENGTH: usize = 64;
const LABEL_LENGTH: usize = 18;
const OPTIONS_OFFSET: u16 = 3;
const NUMBERS: [&str; 14] = [
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "10",
    "11",
    "12",
    "13"
];


pub fn context_menu_tui(
    terminal: &mut DefaultTerminal, selected_weapon: Option<GunData>
) -> (ModdingContext, GunData) {
    let mut app = ContextMenuApp::new(selected_weapon);
    _ = terminal.draw(|frame| app.draw(frame)).unwrap();
    while app.running {
        let event = event::read();
        if let Ok(event) = event {
            match event {
                Event::Mouse(mouse_event) => {
                    app.handle_mouse_event(mouse_event);
                },
                Event::Key(key_event) => {
                    if key_event.code == KeyCode::Esc {
                        app.running = false;
                    } else if key_event.code == KeyCode::Enter {
                        app.go_to = Some(GoToTerm::SubmitBuild);
                    }
                }
                Event::Resize(_, _) => {
                    app.redraw = true;
                },
                _ => {}
            }
        }
        if let Some(go_to) = app.go_to {
            app.hovered_row = 0;
            match go_to {
                GoToTerm::WeaponSelect => {
                    let new_selection = weapon_search_tui(terminal, app.weapon_selection);
                    app.weapon_selection = new_selection;
                },
                GoToTerm::BuffStats => {
                    let new_buffs = stat_screen_tui(terminal, app.buff_stats, false);
                    app.buff_stats = new_buffs;
                },
                GoToTerm::RivenStats => {
                    let new_riven = stat_screen_tui(terminal, app.riven_stats, true);
                    app.riven_stats = new_riven;
                },
                GoToTerm::SubmitBuild => {
                    if let Some(gun_data) = &app.weapon_selection {
                        let modding_context = app.get_modding_context();
                        let mod_sums = app.get_mod_sums(&modding_context);
                        let loaded_mods = app.get_loaded_mods(&modding_context);
                        build_display_tui(terminal, gun_data, modding_context, loaded_mods, Some(mod_sums));
                    }
                }
            }
            app.go_to = None;
        }
        if app.redraw {
            _ = terminal.draw(|frame| app.draw(frame)).unwrap();
            app.redraw = false;
        }
    }
    (app.get_modding_context(), app.weapon_selection.unwrap())
}


#[derive(Copy, Clone, Eq, PartialEq)]
enum GoToTerm {
    WeaponSelect,
    BuffStats,
    RivenStats,
    SubmitBuild
}


struct ContextMenuApp {
    weapon_selection: Option<GunData>,
    damage_criteria: DamageCriteria,
    hovered_row: u16,
    has_kills: bool,
    ads: bool,
    acuity: bool,
    amalgam: bool,
    bane: u8,
    status_count: u8,
    running: bool,
    redraw: bool,
    go_to: Option<GoToTerm>,
    buff_stats: Option<StatFields>,
    riven_stats: Option<StatFields>,
} impl ContextMenuApp {

    fn get_mod_sums(&self, modding_context: &ModdingContext) -> GunModSums {
        let mut sums = GunModSums::new();
        if modding_context.conditions > 0 {
            sums.conditions = modding_context.conditions;
        }
        sums
    }

    fn get_loaded_mods(&self, modding_context: &ModdingContext) -> LoadedMods {
        let mut loaded_mods = LoadedMods::new(modding_context);
        if let Some(riven_stats) = &self.riven_stats {
            let all = riven_stats.get_all();
            let mut riven_data = ModData::new();
            for &(stat, value) in all {
                if value == 0 {
                    continue;
                } else {
                    riven_data.push(stat, value);
                }
                if riven_data.count >= 4 {
                    break;
                }
            }
            loaded_mods.update_riven(riven_data);
        }
        loaded_mods
    }
    
    fn get_modding_context(&self) -> ModdingContext {
        let (gun_type, semi) = if let Some(g) = &self.weapon_selection {
            (g.gun_stats.gun_type, g.semi)
        } else {
            (WeaponType::All, false)
        };
        ModdingContext {
            weapon_type: gun_type,
            damage_criteria: self.damage_criteria,
            kills: self.has_kills,
            aiming: self.ads,
            semi,
            acuity: self.acuity,
            prefer_amalgam: self.amalgam,
            riven: self.riven_stats.is_some(),
            bane: self.bane > 0,
            prime_bane: self.bane > 1,
            buffs: self.buff_stats.is_some(),
            conditions: self.status_count
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        // update mouse_row
        let row = mouse_event.row;
        if row != self.hovered_row {
            self.hovered_row = row;
            self.redraw = true;
        }
        // fetch hovered field
        let field = if let Some(field) = FieldType::get_type(row) {
            field
        } else {
            return;
        };
        // check for clicking
        let clicked_side: i8 = clicked(mouse_event.kind);
        // interpret click
        if clicked_side != 0 {
            self.click(field, clicked_side > 0);
            self.redraw = true;
        }
    }

    fn click(&mut self, field: FieldType, left: bool) {
        match field {
            FieldType::SelectedWeapon => {
                if left {
                    self.go_to = Some(GoToTerm::WeaponSelect);
                } else {
                    self.weapon_selection = None;
                };
            },
            FieldType::DamageCriteria => {
                if left {
                    match self.damage_criteria {
                        DamageCriteria::PerShot => {
                            self.damage_criteria = DamageCriteria::BurstDPS;
                        },
                        DamageCriteria::BurstDPS => {
                            self.damage_criteria = DamageCriteria::SustainedDPS;
                        },
                        DamageCriteria::SustainedDPS => {
                            self.damage_criteria = DamageCriteria::PerShot;
                        }
                    }
                } else {
                    self.damage_criteria = DamageCriteria::SustainedDPS;
                }
            },
            FieldType::HasKills => {
                if left {
                    self.has_kills = !self.has_kills;
                } else {
                    self.has_kills = true;
                }
            },
            FieldType::ADS => {
                if left {
                    self.ads = !self.ads;
                } else {
                    self.ads = true;
                }
            },
            FieldType::Acuity => {
                if left {
                    self.acuity = !self.acuity;
                } else {
                    self.acuity = false;
                }
            },
            FieldType::Amalgam => {
                if left {
                    self.amalgam = !self.amalgam;
                } else {
                    self.amalgam = false;
                }
            },
            FieldType::BaneMods => {
                if left {
                    self.bane +=1;
                    if self.bane >= 3 {
                        self.bane = 0;
                    }
                } else {
                    self.bane = 0;
                }
            },
            FieldType::StatusCount => {
                if left {
                    self.status_count += 1;
                    if self.status_count > 13 {
                        self.status_count = 0;
                    }
                } else if self.status_count == 0 {
                    self.status_count = 13;
                } else {
                    self.status_count -= 1;
                }
            },
            FieldType::AppliedBuffs => {
                if left {
                    self.go_to = Some(GoToTerm::BuffStats);
                } else {
                    self.buff_stats = None;
                };
            },
            FieldType::RivenStats => {
                if left {
                    self.go_to = Some(GoToTerm::RivenStats);
                } else {
                    self.riven_stats = None;
                };
            }
        }
    }

    // fn click(&mut self, field: FieldType, left: bool) {
    //     match field {
    //         FieldType::SelectedWeapon => { self.go_to_weapon_select = true; },
    //         FieldType::DamageCriteria => {},
    //         FieldType::HasKills => {},
    //         FieldType::ADS => {},
    //         FieldType::Acuity => {},
    //         FieldType::Amalgam => {},
    //         FieldType::BaneMods => {},
    //         FieldType::StatusCount => {},
    //         FieldType::AppliedBuffs => {},
    //         FieldType::RivenStats => {}
    //     }
    // }

    fn new(selected_weapon: Option<GunData>) -> Self {
        Self {
            weapon_selection: selected_weapon,
            damage_criteria: DamageCriteria::default(),
            hovered_row: 0,
            has_kills: true,
            ads: true,
            acuity: false,
            amalgam: false,
            bane: 0,
            status_count: 0,
            running: true,
            redraw: false,
            go_to: None,
            buff_stats: None,
            riven_stats: None
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical_layout = Layout::vertical([
            Length(2),
            Min(FieldType::COUNT as u16 + 2),
            Length(3)
        ]);
        let [help_area, field_area, buttons_area] = vertical_layout.areas(frame.area());
        // ]).split(frame.area());
        // let buttons_layout = Layout::horizontal([
        //     Fill(1),
        //     Percentage(20)
        // ]).split(vertical_layout[1]);
        self.draw_instructions(frame, help_area);
        self.draw_fields(frame, field_area);
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
        );
        frame.render_widget(list, area);
    }

    fn draw_fields(&mut self, frame: &mut Frame, area: Rect) {
        let mut fields = Vec::with_capacity(FieldType::COUNT);
        for field_type in FieldType::FIELDS {
            let mut field_string = field_type.get_label();
            self.push_field_content(field_type, &mut field_string);
            let content = Line::from(Span::styled(field_string, self.get_row_style(field_type)));
            let line = ListItem::new(content);
            // let line = ListItem::new(field_string).style(self.get_row_style(field_type));
            fields.push(line);
        };
        frame.render_widget(List::new(fields).block(Block::bordered().title("Settings")), area);
    }

    fn get_row_style(&self, rendered_field: FieldType) -> Style {
        if let Some(field) = FieldType::get_type(self.hovered_row) {
            if rendered_field == field {
                Style::default().reversed()
            } else {
                Style::default()
            }
        } else {
            Style::default()
        }
    }

    fn push_field_content(&self, field_type: FieldType, field_string: &mut String) {
        fn bool_str(bool: bool) -> &'static str {
            if bool {
                "Yes"
            } else {
                "No"
            }
        }
        match field_type {
            FieldType::SelectedWeapon => {
                if let Some(gun_data) = &self.weapon_selection {
                    field_string.push_str(gun_data.name);
                    field_string.push_str("; ");
                    field_string.push_str(gun_data.fire_mode);
                } else {
                    field_string.push_str("None selected; click to edit.");
                }
            },
            FieldType::DamageCriteria => {
                field_string.push_str(self.damage_criteria.str());
            },
            FieldType::HasKills => {
                field_string.push_str(bool_str(self.has_kills));
            },
            FieldType::ADS => {
                field_string.push_str(bool_str(self.ads));
            },
            FieldType::Acuity => {
                field_string.push_str(bool_str(self.acuity));
            },
            FieldType::Amalgam => {
                let s = if self.amalgam {
                    if let Some(gun_data) = &self.weapon_selection {
                        gun_data.gun_stats.gun_type.amalgam()
                    } else {
                        "Use Amalgam mod"
                    }
                } else {
                    "Don't use Amalgam mod"
                };
                field_string.push_str(s);
            },
            FieldType::BaneMods => {
                let s =match self.bane {
                    1 => "Use Bane mod",
                    2 => "Use Primed Bane mod",
                    _ => "Don't use Bane mods"
                };
                field_string.push_str(s);
            },
            FieldType::StatusCount => {
                field_string.push_str(NUMBERS[self.status_count as usize]);
            },
            FieldType::AppliedBuffs => {
                if let Some(stat_fields) = &self.buff_stats {
                    let display_string = stat_fields.display();
                    field_string.push_str(&display_string);
                } else {
                    field_string.push_str("None; click to edit");
                }
            },
            FieldType::RivenStats => {
                if let Some(stat_fields) = &self.riven_stats {
                    let display_string = stat_fields.display();
                    field_string.push_str(&display_string);
                } else {
                    field_string.push_str("None; click to edit");
                }
            }
        }
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
    RivenStats
} impl FieldType {

    const COUNT: usize = 10;
    const MAX: u16 =  OPTIONS_OFFSET + Self::COUNT as u16;
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
        Self::RivenStats
    ];

    fn get_type(row: u16) -> Option<Self> {
        let n = row;
        if (OPTIONS_OFFSET..Self::MAX).contains(&n) {
            Some(Self::FIELDS[(n-OPTIONS_OFFSET) as usize])
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
            Self::RivenStats => "Riven Mod"
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
