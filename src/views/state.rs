use tui::widgets::ListState;

#[derive(Default)]
pub struct FileState {
    pub file_hierarchy: String
}

pub enum View {
    FileView(FileState),
    TagView
}

pub struct ProgramState<'a> {
    pub view: View,
    pub list_state: &'a mut ListState
}
