use crate::config::Config;
use crate::data::Knowledge;
use crate::views::file_view;
use crate::views::state;
use crate::views::state::{App, FileMode, Tab};
use std::fs::read_dir;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color::Black;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;

pub fn draw_views<T: Backend>(f: &mut Frame<T>, app: &mut App) {
    if let Some(state) = app.get_latest_state() {
        match state {
            state::ViewState::FileView => {
                draw_files_view(f, app);
            }
            state::ViewState::AddView => {
                draw_add_view(f, app);
            }
            _ => {}
        }
    } else {
        panic!("Don't have any state in the stack to draw!");
    }
}

macro_rules! all_files {
    ($file:expr) => {{
        let item: Vec<_> = std::fs::read_dir($file)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        item
    }};
}

pub fn draw_files_view<T: Backend>(f: &mut Frame<T>, app: &mut App) {
    // TODO: handle multiple data directories
    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("knowledge-base!")
        .border_type(BorderType::Rounded);
    match app.file_mode {
        state::FileMode::Dir => {
            let left_paths: Vec<_> = app
                .files
                .iter()
                .map(|e| ListItem::new(Span::from(Span::styled(e.clone(), Style::default()))))
                .collect();
            let selected_file = app.files.get(app.file_cycle.current_item).unwrap();

            let mut selected_file_path = app.base_path.clone();
            selected_file_path.push(selected_file);
            let right_item_text = if selected_file_path.is_dir() {
                let item: Vec<String> = all_files!(selected_file_path);
                Text::from(item.join("\n"))
            } else if selected_file_path.is_file() {
                let knowledge = Knowledge::from_file(selected_file_path);
                Text::from(knowledge.text)
            } else {
                Text::from("error in reading files")
            };
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());
            let descriptions_widget =
                Paragraph::new(right_item_text).block(main_block.clone().title("text"));
            let main_list = List::new(left_paths)
                .block(main_block.clone())
                .highlight_style(
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );
            let mut list_state = ListState::default();
            list_state.select(Some(app.file_cycle.current_item));
            f.render_stateful_widget(main_list, chunks[0], &mut list_state);
            f.render_widget(descriptions_widget, chunks[1])
        }
        state::FileMode::File => {
            // create one view
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());
            let knowledge = Knowledge::from_file(&app.base_path);
            let title_widget = Paragraph::new(Text::from(knowledge.title.as_ref()))
                .block(main_block.clone().title("Title"));
            let content_widget = Paragraph::new(Text::from(knowledge.text))
                .block(main_block.clone().title("Content"));
            f.render_widget(title_widget, chunks[0]);
            f.render_widget(content_widget, chunks[1]);
        }
    }
}
pub fn draw_add_view<T: Backend>(f: &mut Frame<T>, app: &mut App) {
    let default_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(f.size());
    let mut title_block = default_block.clone().title("Title");
    let mut tag_block = default_block.clone().title("Tag");
    let mut text_block = default_block.clone().title("Text");
    if let Some(current_tab) = app.input_tabs.get(app.input_current_tab.current_item) {
        match current_tab {
            Tab::Title => {
                title_block = title_block.border_style(Style::default().fg(Color::Cyan));
            }
            Tab::Text => {
                text_block = text_block.border_style(Style::default().fg(Color::Cyan));
            }
            Tab::Tags => {
                tag_block = tag_block.border_style(Style::default().fg(Color::Cyan));
            }
        }
    }
    let title = Paragraph::new(app.input_title.get_string()).block(title_block);
    let tag = Paragraph::new(app.input_tags.get_string()).block(tag_block);
    let text = Paragraph::new(app.input_text.get_string()).block(text_block);
    f.render_widget(title, chunks[0]);
    f.render_widget(tag, chunks[1]);
    f.render_widget(text, chunks[2]);
}
