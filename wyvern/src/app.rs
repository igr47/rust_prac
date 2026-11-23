use std::sync::mpsc::{self, Receiver};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    layout::{Position, Rect},
};

use crate::{
    backend::{Backend, BackendEvent},
    commands::{self, ShellCommand},
    theme::{Theme, ThemeKind},
};

const MAX_RESULT_LINES: usize = 5000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Focus {
    Input,
    Results,
    Tables,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineKind {
    Info,
    Success,
    Error,
    Warn,
    Header,
    Row,
    Dim,
    Echo,
}

pub struct ResultLine {
    pub kind: LineKind,
    pub text: String,
}

pub struct DbEntry {
    pub name: String,
    pub tables: Vec<String>,
}

/// Panel rectangles from the last draw (used for mouse hit-testing).
#[derive(Default, Clone, Copy)]
pub struct Areas {
    pub tables: Rect,
    pub results: Rect,
    pub input: Rect,
}

pub struct App {
    // look & feel
    pub theme_kind: ThemeKind,
    pub transparent: bool,
    pub focus: Focus,
    pub tick: u64,
    pub should_quit: bool,
    pub areas: Areas,

    // input area
    pub input: Vec<char>,
    pub cursor: usize,
    pub history: Vec<String>,
    /// Scroll of the input log, in lines up from the bottom (0 = follow).
    pub input_scroll: usize,
    hist_pos: Option<usize>,
    draft: String,

    // result area
    pub results: Vec<ResultLine>,
    /// Lines up from the bottom (0 = follow newest output).
    pub results_scroll: usize,
    pub show_splash: bool,

    // tables / databases panel
    pub catalog: Vec<DbEntry>,
    pub tables_scroll: usize,
    pub current_db: Option<String>,

    // backend
    backend: Backend,
    rx: Receiver<BackendEvent>,
    jobs: usize,
}

impl App {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let db = |n: &str, t: &[&str]| DbEntry {
            name: n.into(),
            tables: t.iter().map(|s| s.to_string()).collect(),
        };
        Self {
            theme_kind: ThemeKind::Fire,
            transparent: false,
            focus: Focus::Input,
            tick: 0,
            should_quit: false,
            areas: Areas::default(),
            input: Vec::new(),
            cursor: 0,
            history: Vec::new(),
            input_scroll: 0,
            hist_pos: None,
            draft: String::new(),
            results: Vec::new(),
            results_scroll: 0,
            show_splash: true,
            // Test data for the tables panel
            catalog: vec![
                db("demo", &["dragons", "wyverns", "hoards", "riders", "eggs", "lairs"]),
                db("archive", &["scrolls", "prophecies"]),
                db("sandbox", &[]),
            ],
            tables_scroll: 0,
            current_db: Some("demo".into()),
            backend: Backend::new(tx),
            rx,
            jobs: 0,
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::new(self.theme_kind, self.transparent)
    }

    pub fn is_running(&self) -> bool {
        self.jobs > 0
    }

    // ───────────────────────── results ─────────────────────────

    pub fn push(&mut self, kind: LineKind, text: impl Into<String>) {
        self.show_splash = false;
        self.results.push(ResultLine { kind, text: text.into() });
        // If the user scrolled up, keep their view steady while output grows.
        if self.results_scroll > 0 {
            self.results_scroll += 1;
        }
        if self.results.len() > MAX_RESULT_LINES {
            let extra = self.results.len() - MAX_RESULT_LINES;
            self.results.drain(..extra);
        }
    }

    /// Drain everything the backend has produced since last frame.
    pub fn poll_backend(&mut self) {
        while let Ok(ev) = self.rx.try_recv() {
            match ev {
                BackendEvent::Line(kind, text) => self.push(kind, text),
                BackendEvent::DatabaseCreated(name) => {
                    if !self.catalog.iter().any(|d| d.name == name) {
                        self.catalog.push(DbEntry { name, tables: vec![] });
                    }
                }
                BackendEvent::TableCreated { db, table } => {
                    if let Some(entry) = self.catalog.iter_mut().find(|d| d.name == db) {
                        entry.tables.push(table);
                    }
                }
                BackendEvent::DbSelected(name) => {
                    if self.catalog.iter().any(|d| d.name == name) {
                        self.push(LineKind::Success, format!("✔ now using `{name}`"));
                        self.current_db = Some(name);
                    } else {
                        self.push(LineKind::Error, format!("✘ no such database `{name}`"));
                    }
                }
                BackendEvent::Finished => self.jobs = self.jobs.saturating_sub(1),
            }
        }
    }

    // ───────────────────────── scrolling ─────────────────────────

    /// `up` > 0 moves toward older content; < 0 toward newer.
    fn scroll(&mut self, region: Focus, up: isize) {
        match region {
            Focus::Results => self.results_scroll = add_signed(self.results_scroll, up),
            Focus::Input => self.input_scroll = add_signed(self.input_scroll, up),
            // Tables list is top-anchored, so "up" means a smaller offset.
            Focus::Tables => self.tables_scroll = add_signed(self.tables_scroll, -up),
        }
        // Over-scrolling is clamped at draw time.
    }

    fn page(&self) -> isize {
        (self.areas.results.height.saturating_sub(3) as isize).max(1)
    }

    pub fn on_mouse(&mut self, m: MouseEvent) {
        let pos = Position::new(m.column, m.row);
        let region = if self.areas.results.contains(pos) {
            Focus::Results
        } else if self.areas.tables.contains(pos) {
            Focus::Tables
        } else if self.areas.input.contains(pos) {
            Focus::Input
        } else {
            return;
        };
        match m.kind {
            MouseEventKind::Down(MouseButton::Left) => self.focus = region,
            MouseEventKind::ScrollUp => self.scroll(region, 3),
            MouseEventKind::ScrollDown => self.scroll(region, -3),
            _ => {}
        }
    }

    // ───────────────────────── keyboard ─────────────────────────

    pub fn on_key(&mut self, key: KeyEvent) {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

        match (key.code, ctrl) {
            (KeyCode::Char('c' | 'd'), true) => self.should_quit = true,
            (KeyCode::Char('l'), true) => self.run_shell(ShellCommand::ClearResults),
            (KeyCode::Char('k'), true) => self.run_shell(ShellCommand::ClearInput),
            (KeyCode::Char('t'), true) => {
                self.theme_kind = self.theme_kind.next();
            }
            (KeyCode::Char('b'), true) => self.transparent = !self.transparent,
            (KeyCode::Tab, _) => {
                self.focus = match self.focus {
                    Focus::Input => Focus::Results,
                    Focus::Results => Focus::Tables,
                    Focus::Tables => Focus::Input,
                }
            }
            (KeyCode::BackTab, _) => {
                self.focus = match self.focus {
                    Focus::Input => Focus::Tables,
                    Focus::Tables => Focus::Results,
                    Focus::Results => Focus::Input,
                }
            }
            (KeyCode::Esc, _) => self.focus = Focus::Input,
            (KeyCode::PageUp, _) => {
                let region = if self.focus == Focus::Input { Focus::Results } else { self.focus };
                self.scroll(region, self.page());
            }
            (KeyCode::PageDown, _) => {
                let region = if self.focus == Focus::Input { Focus::Results } else { self.focus };
                self.scroll(region, -self.page());
            }
            _ => match self.focus {
                Focus::Input => self.on_key_input(key, ctrl),
                other => self.on_key_scroll(other, key),
            },
        }
    }

    fn on_key_scroll(&mut self, region: Focus, key: KeyEvent) {
        let up = match key.code {
            KeyCode::Up | KeyCode::Char('k') => 1,
            KeyCode::Down | KeyCode::Char('j') => -1,
            KeyCode::Home => 1_000_000,
            KeyCode::End => -1_000_000,
            _ => return,
        };
        self.scroll(region, up);
    }

    fn on_key_input(&mut self, key: KeyEvent, ctrl: bool) {
        match (key.code, ctrl) {
            (KeyCode::Enter, _) => self.submit(),
            (KeyCode::Char('u'), true) => {
                self.input.clear();
                self.cursor = 0;
            }
            (KeyCode::Char('a'), true) | (KeyCode::Home, _) => self.cursor = 0,
            (KeyCode::Char('e'), true) | (KeyCode::End, _) => self.cursor = self.input.len(),
            (KeyCode::Up, true) => self.scroll(Focus::Input, 1),
            (KeyCode::Down, true) => self.scroll(Focus::Input, -1),
            (KeyCode::Up, _) => self.history_prev(),
            (KeyCode::Down, _) => self.history_next(),
            (KeyCode::Left, _) => self.cursor = self.cursor.saturating_sub(1),
            (KeyCode::Right, _) => self.cursor = (self.cursor + 1).min(self.input.len()),
            (KeyCode::Backspace, _) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.input.remove(self.cursor);
                    self.edited();
                }
            }
            (KeyCode::Delete, _) => {
                if self.cursor < self.input.len() {
                    self.input.remove(self.cursor);
                    self.edited();
                }
            }
            (KeyCode::Char(c), false) => {
                self.input.insert(self.cursor, c);
                self.cursor += 1;
                self.edited();
            }
            _ => {}
        }
    }

    fn edited(&mut self) {
        self.hist_pos = None;
        self.input_scroll = 0;
    }

    // ───────────────────────── input history ─────────────────────────

    fn input_string(&self) -> String {
        self.input.iter().collect()
    }

    fn set_input(&mut self, s: &str) {
        self.input = s.chars().collect();
        self.cursor = self.input.len();
    }

    fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let pos = match self.hist_pos {
            None => {
                self.draft = self.input_string();
                self.history.len() - 1
            }
            Some(0) => 0,
            Some(i) => i - 1,
        };
        self.hist_pos = Some(pos);
        let s = self.history[pos].clone();
        self.set_input(&s);
    }

    fn history_next(&mut self) {
        match self.hist_pos {
            None => {}
            Some(i) if i + 1 < self.history.len() => {
                self.hist_pos = Some(i + 1);
                let s = self.history[i + 1].clone();
                self.set_input(&s);
            }
            Some(_) => {
                self.hist_pos = None;
                let d = std::mem::take(&mut self.draft);
                self.set_input(&d);
            }
        }
    }

    // ───────────────────────── submit / execute ─────────────────────────

    fn submit(&mut self) {
        let line = self.input_string().trim().to_string();
        self.input.clear();
        self.cursor = 0;
        self.hist_pos = None;
        self.draft.clear();
        self.input_scroll = 0;
        if line.is_empty() {
            return;
        }

        self.history.push(line.clone());
        self.push(LineKind::Echo, format!("❯ {line}"));

        match commands::parse(&line) {
            Some(cmd) => self.run_shell(cmd),
            None => {
                self.jobs += 1;
                self.backend.execute(line, self.current_db.clone());
            }
        }
    }

    pub fn run_shell(&mut self, cmd: ShellCommand) {
        match cmd {
            ShellCommand::ClearResults => {
                self.results.clear();
                self.results_scroll = 0;
                self.show_splash = false;
            }
            ShellCommand::ClearInput => {
                self.history.clear();
                self.hist_pos = None;
                self.input_scroll = 0;
            }
            ShellCommand::ClearAll => {
                self.run_shell(ShellCommand::ClearResults);
                self.run_shell(ShellCommand::ClearInput);
            }
            ShellCommand::Theme(None) => {
                self.push(
                    LineKind::Info,
                    format!("current theme: {} ({})", self.theme_kind.name(), self.theme_kind.epithet()),
                );
                self.push(LineKind::Dim, "usage: theme <fire|frost|earth|water|wind|darkness|light>");
            }
            ShellCommand::Theme(Some(name)) => match ThemeKind::parse(&name) {
                Some(k) => {
                    self.theme_kind = k;
                    self.push(LineKind::Success, format!("✔ theme set to {} ({})", k.name(), k.epithet()));
                }
                None => self.push(LineKind::Error, format!("✘ unknown theme `{name}` - try `themes`")),
            },
            ShellCommand::Themes => {
                self.push(LineKind::Header, "Available themes");
                for k in ThemeKind::ALL {
                    let mark = if k == self.theme_kind { "●" } else { "○" };
                    self.push(LineKind::Info, format!("  {mark} {:<9} {}", k.name(), k.epithet()));
                }
            }
            ShellCommand::Transparent(v) => {
                self.transparent = v.unwrap_or(!self.transparent);
                let state = if self.transparent { "transparent" } else { "solid" };
                self.push(LineKind::Success, format!("✔ background is now {state}"));
            }
            ShellCommand::Help => {
                for (kind, text) in commands::help_lines() {
                    self.push(kind, text);
                }
            }
            ShellCommand::Splash => {
                self.results.clear();
                self.results_scroll = 0;
                self.show_splash = true;
            }
            ShellCommand::Quit => self.should_quit = true,
        }
    }
}

fn add_signed(v: usize, d: isize) -> usize {
    if d >= 0 {
        v.saturating_add(d as usize)
    } else {
        v.saturating_sub(d.unsigned_abs())
    }
}

