use btleplug::platform::PeripheralId;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::channel::oneshot::{self};
use log::debug;
use ratatui::widgets::*;

use ratatui::prelude::*;
use std::io::{stdout, Result};

pub struct BlooTui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    devices: Vec<PeripheralId>,
}

pub trait EventSubscriber {
    fn scan_started(&self);
    fn add_device(&mut self, device: PeripheralId);
}

impl BlooTui {
    pub fn new() -> Result<BlooTui> {
        Ok(BlooTui {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            devices: Vec::default(),
        })
    }

    pub async fn start_tui(&mut self) -> Result<()> {
        debug!("Creating crossterm instance 1");

        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        self.terminal.clear()?;
        self.start_event_loop().await?;

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

    async fn start_event_loop(&mut self) -> Result<()> {
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

            let (_, mut rx) = oneshot::channel();
            while let Ok(Some(device)) = rx.try_recv() {
                debug!("Adding device {:?}", device);
                self.devices.append(device);
            }
        }
        Ok(())
    }
}

impl EventSubscriber for BlooTui {
    fn scan_started(&self) {
        println!("Scan started");
    }

    fn add_device(&mut self, id: PeripheralId) {
        self.devices.push(id);
    }
}
