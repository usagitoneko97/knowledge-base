use crate::config::Config;
use crate::data::Knowledge;
use crate::views::state;
use std::fs::read_dir;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Text};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};
use tui::Frame;
use crate::views::file_view;

pub fn draw_views<T: Backend>(
    f: &mut Frame<T>,
    program_state: &mut state::ProgramState,
) {
    match program_state.view_state {
        state::ViewState::FileView(ref mut file_view) => {
            draw_files_view(f, file_view)
        }
        _ => {}
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

pub fn draw_files_view<T: Backend>(
    f: &mut Frame<T>,
    file_state: &mut file_view::FileState
) {
    // TODO: handle multiple data directories
    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("knowledge-base!")
        .border_type(BorderType::Rounded);
    match file_state.mode {
        file_view::FileMode::Dir => {
            let left_paths: Vec<_> = file_state
                .files
                .iter()
                .map(|e| ListItem::new(Span::from(Span::styled(e.clone(), Style::default()))))
                .collect();
            let selected_file = file_state
                .files
                .get(file_state.cycle.current_item)
                .unwrap();

            let mut selected_file_path = file_state.base_path.clone();
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
            list_state.select(Some(file_state.cycle.current_item));
            f.render_stateful_widget(main_list, chunks[0], &mut list_state);
            f.render_widget(descriptions_widget, chunks[1])
        }
        file_view::FileMode::File => {
            // create one view
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());
            let knowledge = Knowledge::from_file(&file_state.base_path);
            let title_widget = Paragraph::new(Text::from(knowledge.title.as_ref()))
                .block(main_block.clone().title("Title"));
            let content_widget = Paragraph::new(Text::from(knowledge.text))
                .block(main_block.clone().title("Content"));
            f.render_widget(title_widget, chunks[0]);
            f.render_widget(content_widget, chunks[1]);
        }
    }
}
