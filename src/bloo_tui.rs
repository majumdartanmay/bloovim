use btleplug::platform::PeripheralId;
use std::sync::{Arc, Mutex};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::debug;
use ratatui::widgets::*;

use ratatui::prelude::*;
use std::io::{stdout, Result};
use tokio::sync::mpsc;

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
        rx: Arc<Mutex<mpsc::Receiver<PeripheralId>>>,
        devices_arc: Arc<Mutex<Vec<PeripheralId>>>,
    ) -> Result<()> {
        debug!("Creating crossterm instance 1....");

        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        self.terminal.clear()?;

        debug!("Creating device vector. State 1");
        let mut device_raw1: Vec<PeripheralId> = devices_arc.lock().unwrap().to_vec();

        debug!("Attempting to start event loop");
        // let f1 = self.start_event_loop(&mut device_raw1);

        debug!("Attempting to create receiver listener");
        // let f2 = tokio::spawn(async move {
        //     while let Some(data) = rx.lock().unwrap().blocking_recv() {
        //         debug!("Received some perpheral packets");
        //         let mut devices = devices_arc.lock().unwrap().to_vec();
        //         devices.push(data);
        //     }
        // });
        //
        let f2 = std::thread::spawn(move || {
            // while let Some(data) = rx.lock().unwrap().blocking_recv() {}

            loop {
                if let Some(data) = rx.lock().unwrap().blocking_recv() {
                    debug!("received some perpheral packets");
                    let mut devices = devices_arc.lock().unwrap().to_vec();
                    devices.push(data);
                }
            }
        });

        debug!("Joining receiver listener and UI thread");
        // let _ = tokio::join!(f1, f2);
        // let _ = tokio::join!(f2);
        f2.join().expect("Receiver thread panicked");
        debug!("Closing receiver listener and UI thread");
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

    async fn start_event_loop(&mut self, devices: &mut Vec<PeripheralId>) -> Result<()> {
        loop {
            // draw UI
            // self.terminal.draw(|frame: &mut Frame| {
            //     Self::render_list(frame, devices);
            // })?;

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
