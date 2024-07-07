use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub trait Page {
    type RunResult;

    async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Self::RunResult; 
    async fn handle_events(&mut self);
}