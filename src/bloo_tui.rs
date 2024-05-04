use btleplug::platform::PeripheralId;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::widgets::*;

use ratatui::prelude::*;
use std::io::{stdout, Result};

pub struct TuiController {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    devices: Vec<PeripheralId>,
}

pub trait EventSubscriber {
    fn scan_started(&self);
    fn add_device(&mut self, device: PeripheralId);
}

impl TuiController {
    pub fn new() -> Result<TuiController> {
        Ok(TuiController {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            devices: Vec::default(),
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

    fn render_list(_frame: &mut Frame, devices: &mut [PeripheralId]) {
        let items2 = devices
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let list = List::new(items2)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        _frame.render_widget(list, _frame.size());
    }

    fn start_event_loop(&mut self) -> Result<()> {
        loop {
            // draw UI
            self.terminal.draw(|frame: &mut Frame| {
                Self::render_list(frame, &mut self.devices);
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

impl EventSubscriber for TuiController {
    fn scan_started(&self) {
        println!("Scan started");
    }

    fn add_device(&mut self, id: PeripheralId) {
        self.devices.push(id);
    }
}
