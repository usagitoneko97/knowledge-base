use tui::Frame;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::text::{Span, Text};
use tui::widgets::{ListItem, Paragraph, List, Block, Borders, BorderType};
use tui::style::{Style, Color, Modifier};
use crate::config::Config;
use crate::views::state;
use std::fs::read_dir;
use crate::data::Knowledge;
use tui::layout::{Layout, Direction, Constraint};


macro_rules! all_files {
    ($file:expr) => {{
        let item: Vec<_> = std::fs::read_dir($file).unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        item
    }
    };
}

fn draw_files_view<T: Backend>(mut f: Frame<T>, config: &Config, state: &mut state::ProgramState) {
    // TODO: handle multiple data directories
    let mut file_path = PathBuf::from(config.data_directories.last().unwrap().as_str());
    match &state.view {
        state::View::FileView(file_view) => {
            file_path.push(file_view.file_hierarchy.clone());
        }
        state::View::TagView => {}
    }
    let left_paths: Vec<_> = if let Ok(file_entry) = std::fs::read_dir(&file_path) {
        file_entry
            .filter_map(|e| {
                let dir_entry = e.unwrap();
                return match &dir_entry.file_type() {
                    Ok(file) => {
                        if file.is_file() {
                            if dir_entry.path().extension().unwrap() == "md" {
                                ()
                            }
                        }
                        Some(ListItem::new(Span::from(Span::styled(dir_entry
                                                                       .file_name()
                                                                       .into_string()
                                                                       .unwrap(),
                                                                   Style::default()))))
                    }
                    Err(e) => None
                }
            }).collect()
    } else {vec![]};
    let mut current_file_list: Vec<_> = std::fs::read_dir(&file_path).unwrap()
        .map(|e| e.unwrap()).collect();
    current_file_list.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());
    let selected_file = current_file_list
        .get(state.list_state.selected().unwrap())
        .unwrap();
    let right_item_text = if let Ok(entry_type) = selected_file.file_type() {
        if entry_type.is_dir() {
            let item: Vec<String> = all_files!(selected_file.path());
            Text::from(item.join("\n"))
        } else if entry_type.is_file() {
            let knowledge = Knowledge::from_file(selected_file.path());
            Text::from(knowledge.text)
        } else {
            Text::from("error in reading files")
        }
    } else {Text::from("error in getting the file type!")};
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("knowledge-base!")
        .border_type(BorderType::Rounded);
    let descriptions_widget = Paragraph::new(right_item_text).block(main_block.clone().title("text"));
    let main_list = List::new(left_paths).block(main_block.clone())
        .highlight_style(Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD));
    f.render_stateful_widget(main_list, chunks[0], &mut state.list_state);
    f.render_widget(descriptions_widget, chunks[1])
}