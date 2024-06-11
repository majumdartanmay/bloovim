use crate::BState;

use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::api::Peripheral;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use futures::StreamExt;
use log::debug;
use ratatui::widgets::*;

use ratatui::prelude::*;
use std::io::{stdout, Result};

pub struct BlooTui {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl BlooTui {
    pub fn new() -> Result<BlooTui> {
        Ok(BlooTui {
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }

    pub async fn start_tui(
        &mut self,
        devices_arc: &mut Vec<String>,
        b_state: &BState,
    ) -> Result<()> {
        debug!("Creating crossterm instance 1....");

        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        self.terminal.clear()?;

        self.start_event_loop(devices_arc, b_state).await?;
        self.stop_tui()?;

        Ok(())
    }

    pub fn stop_tui(&self) -> Result<()> {
        debug!("Attempting to stop TUI");

        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn render_list(_frame: &mut Frame, devices: &mut [String]) {
        let items2 = devices
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let list = List::new(items2)
            .block(
                Block::default()
                    .title("Bluetooth devices")
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        _frame.render_widget(list, _frame.size());
    }

    async fn start_event_loop(
        &mut self,
        devices: &mut Vec<String>,
        b_state: &BState,
    ) -> Result<()> {
        loop {
            // draw UI
            self.terminal.draw(|frame: &mut Frame| {
                Self::render_list(frame, &mut devices.to_vec());
            })?;

            let mut events = b_state.central.events().await.unwrap();
            if let Some(event) = events.next().await {
                debug!(
                    "Some bluetooth event has occured. Event information{:?} ",
                    event
                );
                if let CentralEvent::DeviceDiscovered(id) = event {
                    if let Ok(device_full_info) = b_state.central.as_ref().peripheral(&id).await {
                        let device_name = device_full_info
                            .properties()
                            .await
                            .unwrap()
                            .unwrap()
                            .local_name
                            .unwrap();
                        debug!("Full device info {:?}", device_full_info);
                        if !devices.contains(&device_name) {
                            devices.push(device_name);
                        }
                    }
                }
            }

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
        debug!("Outside event loop");
        Ok(())
    }
}
