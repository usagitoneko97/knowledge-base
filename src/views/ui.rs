use crate::data::Handler;
use crate::nav;
use crate::views::app::App;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;
use tui::Terminal;

enum Event<I> {
    Input(I),
    Tick,
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
    let mut program_state = App::default().set_file_view(h.config);
    loop {
        match terminal.draw(|rect| {
            nav::draw_views(rect, &mut program_state);
        }) {
            std::io::Result::Ok(()) => {}
            std::io::Result::Err(_e) => {panic!("Error in writing into terminal!");}
        }
        match rx.recv().unwrap() {
            Event::Input(event) => {
                if let KeyCode::Char('q') = event.code {
                    disable_raw_mode().expect("Error in disabling raw mode");
                    match terminal.show_cursor() {
                        std::io::Result::Ok(()) => {}
                        std::io::Result::Err(_e) => {panic!("Error in showing cursor!");}
                    }
                    break;
                }
                program_state.update_state(&event);
            }
            _ => {}
        }
    }
}
