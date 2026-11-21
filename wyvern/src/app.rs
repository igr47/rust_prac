use std::sync::mpsc::{self, Receiver};

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind},
    layout::{Position, Rect},
};


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
