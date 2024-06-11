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

    fn render_list(frame: &mut Frame, devices: &mut [String], state: &mut ListState) {
        let list = List::new(devices.to_vec())
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

        let area: Rect = frame.size();
        frame.render_stateful_widget(list, area, state);
    }

    async fn start_event_loop(
        &mut self,
        devices: &mut Vec<String>,
        b_state: &BState,
    ) -> Result<()> {
        let mut state = ListState::default().with_selected(Some(0));
        loop {
            // draw UI
            self.terminal.draw(|frame: &mut Frame| {
                Self::render_list(frame, &mut devices.to_vec(), &mut state);
            })?;

            let mut events = b_state.central.events().await.unwrap();

            if let Some(CentralEvent::DeviceDiscovered(id)) = events.next().await {
                if let Ok(device_full_info) = b_state.central.as_ref().peripheral(&id).await {
                    let device_name = device_full_info
                        .properties()
                        .await
                        .unwrap()
                        .unwrap()
                        .local_name
                        .unwrap();
                    if !devices.contains(&device_name) {
                        debug!("Full device info {:?}", device_full_info);
                        devices.push(device_name);
                    }
                }
            }

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if key.code == KeyCode::Char('q') {
                            break;
                        } else if key.code == KeyCode::Enter {
                            debug!(
                                "Enter has been pressed. Current item pressed : {}",
                                state.selected().unwrap()
                            )
                        }
                    }
                }
            }
        }
        debug!("Outside event loop");
        Ok(())
    }
}
