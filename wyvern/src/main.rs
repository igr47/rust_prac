//! Wyvern - terminal UI shell (ratatui).
//!
//! Module map:
//!   app.rs      - application state, key/mouse handling, shell commands
//!   backend.rs  - MOCK backend (swap for your real query engine later)
//!   commands.rs - hardcoded shell commands (clear, theme, help, ...)
//!   theme.rs    - the seven elemental themes + transparency
//!   art.rs      - ASCII art (title, header wyvern, burning splash wyvern)
//!   ui.rs       - all drawing code

mod app;
mod art;
mod backend;
mod commands;
mod theme;
mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
        execute,
    },
    DefaultTerminal,
};

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    execute!(io::stdout(), EnableMouseCapture)?;

    let result = run(&mut terminal);

    let _ = execute!(io::stdout(), DisableMouseCapture);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();
    let tick_rate = Duration::from_millis(80); // animation speed
    let mut last_tick = Instant::now();

    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Wait for input, but wake up often so streamed results and the
        // fire animation stay smooth.
        if event::poll(Duration::from_millis(30))? {
            loop {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => app.on_key(key),
                    Event::Mouse(m) => app.on_mouse(m),
                    _ => {}
                }
                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        // Pull any results the backend has produced since the last frame.
        app.poll_backend();

        if last_tick.elapsed() >= tick_rate {
            app.tick = app.tick.wrapping_add(1);
            last_tick = Instant::now();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    /// Renders one frame headlessly: `cargo test -- --nocapture` prints it.
    #[test]
    fn renders_splash_and_results() {
        for (w, h) in [(130u16, 40u16), (70, 20)] {
            let mut term = Terminal::new(TestBackend::new(w, h)).unwrap();
            let mut app = App::new();
            app.tick = 5;
            term.draw(|f| ui::draw(f, &mut app)).unwrap();
            for c in "select * from dragons limit 3".chars() {
                app.on_key(ratatui::crossterm::event::KeyEvent::from(
                    ratatui::crossterm::event::KeyCode::Char(c),
                ));
            }
            app.on_key(ratatui::crossterm::event::KeyCode::Enter.into());
            std::thread::sleep(std::time::Duration::from_millis(500));
            app.poll_backend();
            term.draw(|f| ui::draw(f, &mut app)).unwrap();
            println!("{}", term.backend());
        }
    }
}

