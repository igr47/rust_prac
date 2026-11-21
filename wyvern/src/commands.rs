//! Hardcoded shell-level commands (handled by the UI, never sent to the backend)

use crate::app::LineKind;

#[derive(Debug)]
pub enum ShellCommand {
    ClearResults,
    ClearInput,
    ClearAll,
    Theme(Option<String>),
    Themes,
    Transparent(Option<bool>), // None = toggle,
    Help,
    Splash,
    Quit,
}

/// Returns Some(cmd) if `line` is a shell command, None if it should go to the backend 
pub fn parse(line: &str) -> Option<ShellCommand> {
    let lower = line.trim().to_lowercase();
    let mut parts = lower.split_whitespace();
    let cmd = parts.next()?;
    let arg = parts.next()?;

    match (cmd, arg) {
        ("clear" | "cls", None | Some("results")) => Some(ShellCommand::ClearResults),
        ("clear" | "cls", Some("input" | "history")) => Some(ShellCommand::ClearInput),
        ("clear" | "cls", Some("all")) => Some(ShellCommand::ClearAll),
        ("theme", a) => Some(ShellCommand::Theme(a.map(String::from))),
        ("themes", _) => Some(ShellCommand::Themes),
        ("transparent" | "transparency", None | Some("toggle")) => Some(ShellCommand::Transparent(None)),
        ("transparent" | "transparency", Some("on" | "true" | "yes")) => {
            Some(ShellCommand::Transparent(true))
        }
        ("transparent" | "transparency", Some("off" | "false" | "no")) => {
            Some(ShellCommand::Transparent(Some(false)))
        }
        ("help" | "?", _) => Some(ShellCommand::Help),
        ("splash", _) => Some(ShellCommand::Splash),
        ("exit" | "quit" | ":q", _) => Some(ShellCommand::Quit),
        _ => None,
    }
}

pub fn help_lines() -> Vec<(LineKind, String)> {
    use LineKind::*;
    vec![
        (Header, "Shell commands".into()),
        (Info, "  clear | cls            clear the result display".into()),
        (Info, "  clear input            clear the input history".into()),
        (Info, "  clear all              clear both".into()),
        (Info, "  theme [name]           show / switch theme".into()),
        (Info, "  themes                 list themes".into()),
        (Info, "  transparent [on|off]   toggle transparent background".into()),
        (Info, "  splash                 show the burning wyvern again".into()),
        (Info, "  exit | quit            leave Wyvern".into()),
        (Header, "Mock queries (test data, no real backend yet)".into()),
        (Info, "  select * from dragons [limit N]   streams rows live".into()),
        (Info, "  predict <anything>                streams progress live".into()),
        (Info, "  create table|model <name>         adds to the tables panel".into()),
        (Info, "  create database <name>".into()),
        (Info, "  use <database>".into()),
        (Header, "Keys".into()),
        (Info, "  Enter run   Up/Down history   Tab / Shift+Tab change focus".into()),
        (Info, "  PgUp/PgDn scroll   Home/End jump   mouse wheel scrolls panel".into()),
        (Info, "  Ctrl+L clear results   Ctrl+K clear input   Ctrl+U clear line".into()),
        (Info, "  Ctrl+T next theme   Ctrl+B transparency   Ctrl+Up/Down scroll input log".into()),
        (Info, "  Esc focus input   Ctrl+C / Ctrl+D quit".into()),

    ]
}
