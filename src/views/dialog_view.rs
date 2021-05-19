use crate::key::Key;
use crate::views::app::App;

pub fn handler(app: &mut App, event: &Key) {
    match event {
        Key::Left | Key::Right | Key::Char('h') | Key::Char('l') => {
            app.confirm = !app.confirm;
        }
        Key::Enter => {
            if app.confirm {
                if let Some(action) = app.confirm_action {
                    action(app);
                }
            }
            app.pop_state();
            app.refresh_directory();
        }
        _ => {}
    }
}
