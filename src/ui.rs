use chrono::prelude::*;
use std::time::{Instant, Duration};
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::layout::{Layout, Constraint, Direction};
use tui::widgets::{ListItem, List, Borders, BorderType, ListState, Paragraph};
use tui::text::{Span, Text};
use tui::text::Spans;
use tui::style::{Style, Color, Modifier};
use tui::{
    widgets::{
        Block
    }
};
use crate::data::Handler;
use serde::__private::ser::serialize_tagged_newtype;

enum Event<I> {
    Input(I),
    Tick
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
    let mut terminal = Terminal::new(backend)
        .expect("Error in creating new terminal");
    terminal.clear().expect("Error in clearing terminal");
    let mut knowledge_state = ListState::default();
    knowledge_state.select(Some(0));
    loop {
        terminal.draw(|rect| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(rect.size());
            let main_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("knowledge-base!")
                .border_type(BorderType::Rounded);
            let item_list: Vec<_> = h.datas.values().map(|item| {
                ListItem::new(Span::from(Span::styled(item.title.clone(),
                                                      Style::default())))
            }).collect();
            let values: Vec<_> = h.datas.values().collect();
            let text = Text::from(values[knowledge_state.selected().unwrap()].text.clone());
            let descriptions_widget = Paragraph::new(text).block(main_block.clone().title("text"));
            let main_list = List::new(item_list).block(main_block.clone())
                .highlight_style(Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD));
            rect.render_stateful_widget(main_list, chunks[0], &mut knowledge_state);
            rect.render_widget(descriptions_widget, chunks[1])
        });
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().expect("Error in disabling raw mode");
                    terminal.show_cursor();
                    break;
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if let Some(selected) = knowledge_state.selected() {
                        if selected >= h.datas.len()-1 {
                            knowledge_state.select(Some(0));
                        } else {
                            knowledge_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if let Some(selected) = knowledge_state.selected() {
                        if selected == 0 {
                            knowledge_state.select(Some(h.datas.len()-1));
                        } else {
                            knowledge_state.select(Some(selected - 1));
                        }
                    }
                }
                _ => {}
            }
            Event::Tick => {}
        }
    }
}