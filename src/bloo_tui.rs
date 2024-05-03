use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::prelude::*;
use std::io::{stdout, Result};

pub struct TuiController {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

pub trait EventSubscriber {
    fn scan_started(&self) {
        todo!()
    }
}

impl TuiController {
    pub fn new() -> Result<TuiController> {
        Ok(TuiController {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }

    pub fn start_tui(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        self.terminal.clear()?;
        self.start_event_loop()?;

        Ok(())
    }

    pub fn stop_tui(&self) -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn start_event_loop(&mut self) -> Result<()> {
        loop {
            // draw UI
            self.terminal.draw(|frame: &mut Frame| {
                render_list(frame);
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

fn render_list(_frame: &mut Frame) {
    todo!();
}
