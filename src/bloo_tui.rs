use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Result, Stdout};

pub fn start_tui(adapter_name: &String) -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> =
        Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    start_event_loop(&mut terminal, adapter_name)?;
    Ok(())
}

pub fn stop_tui() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn start_event_loop(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    adapter_name: &String,
) -> Result<()> {
    loop {
        // draw UI
        terminal.draw(|frame: &mut Frame| {
            render_list(frame, adapter_name);
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

fn render_list(frame: &mut Frame, adapter_name: &String) {
    let area = frame.size();
    frame.render_widget(
        Paragraph::new(format!("Hello Ratatui! {adapter_name} (press 'q' to quit)"))
            .white()
            .on_blue(),
        area,
    );
}
