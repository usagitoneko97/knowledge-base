use crate::data::{Handler, Knowledge};
use crate::views::state::{FileState, ProgramState, ViewState};
use crate::nav;
use chrono::prelude::*;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::text::{Span, Text};
use tui::widgets::Block;
use tui::widgets::{BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::Terminal;

enum Event<I> {
    Input(I),
    Tick,
}

enum RightItem<'a> {
    Knowledge(&'a Knowledge),
    Parent(String),
}

fn get_list<'a>(
    mapping: &HashMap<String, Vec<&'a Knowledge>>,
    hierarchy: &Option<String>,
) -> Vec<RightItem<'a>> {
    match hierarchy {
        Some(s) => match mapping.get(s) {
            Some(knowledges) => knowledges
                .iter()
                .map(|k| RightItem::Knowledge(*k))
                .collect(),
            None => {
                vec![]
            }
        },
        None => mapping
            .keys()
            .map(|e| e.clone())
            .map(RightItem::Parent)
            .collect(),
    }
}

pub fn ui(h: Handler) {
    enable_raw_mode().expect("Enabling raw mode!");
    let (tx, rx) = std::sync::mpsc::channel();
    let tick_rate = std::time::Duration::from_millis(200);
    std::thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).expect("works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("error in sending inputs");
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Error in creating new terminal");
    terminal.clear().expect("Error in clearing terminal");
    let mut knowledge_state = ListState::default();
    knowledge_state.select(Some(0));
    let mut program_state = ProgramState::new(ViewState::FileView(FileState::new(h.config)),
                                              &mut knowledge_state);
    loop {
        terminal.draw(|rect| {
           nav::draw_views(rect, &mut program_state);
        });
        match rx.recv().unwrap() {
            Event::Input(event) => {
                if let KeyCode::Char('q') = event.code {
                    disable_raw_mode().expect("Error in disabling raw mode");
                    terminal.show_cursor();
                    break;
                }
                program_state.update_state(&event);
            }
            _ => {} /*
                    Event::Input(event) => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode().expect("Error in disabling raw mode");
                            terminal.show_cursor();
                            break;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if let Some(selected) = knowledge_state.selected() {
                                if selected >= display_list.len() -1 {
                                    knowledge_state.select(Some(0));
                                } else {
                                    knowledge_state.select(Some(selected + 1));
                                }
                            }
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            if let Some(selected) = knowledge_state.selected() {
                                if selected == 0 {
                                    knowledge_state.select(Some(display_list.len()-1));
                                } else {
                                    knowledge_state.select(Some(selected - 1));
                                }
                            }
                        }
                        KeyCode::Enter | KeyCode::Char('l') => {
                            hierarchy_state = match display_list.get(knowledge_state.selected().unwrap()) {
                                Some(e) => {
                                    match e {
                                        RightItem::Knowledge(k) => {
                                            hierarchy_state
                                        }
                                        RightItem::Parent(p) => {
                                            Some(p.clone())
                                        }
                                    }
                                }
                                None => {
                                    hierarchy_state
                                }
                            };
                        }
                        KeyCode::Char('h') => {
                            hierarchy_state = None;
                        }
                        _ => {}
                    }
                    Event::Tick => {}
                     */
        }
    }
}
