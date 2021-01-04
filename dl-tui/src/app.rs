pub mod views;
pub mod mode;

use crate::user::User;
use self::{
    views::Views,
    mode::Mode,
};

use rustyline::{
    Cmd, Word, Editor, Movement, line_buffer::LineBuffer,
    config::Config, completion::{Completer, Candidate},
};
use tui::{
    backend::Backend,
    buffer::Buffer,
    terminal::{Terminal, TerminalOptions, Frame},
};

#[derive(Debug, Clap)]
pub struct App {
    user: User,
    cmd: LineBuffer,
    state: AppState,
}

#[derive(Debug)]
pub struct AppState {
    view: Views,
    mode: Mode,
}

impl AppState {
}


impl Default for AppState {
    fn default() -> Self {
        Self {
            view: Views::Home(views::HomeView {}),
            mode: Mode::Command,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            user: User::default(),
            cmd: LineBuffer::with_capacity(4096),
            state: AppState::default(),
        }
    }
}
