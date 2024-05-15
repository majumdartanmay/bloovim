use log::error;
use tokio::sync::mpsc::Sender;

use crate::BState;
use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::platform::Manager;
use btleplug::platform::PeripheralId;
use btleplug::{api::Manager as _, Result};
use futures::StreamExt;
use log::debug;

pub async fn start_bluetooth_stream<'a>(
    sender: &Sender<PeripheralId>,
    b_state: &'a mut BState,
) -> Result<()> {
    debug!("Entered bluetooth scan zone");

    let manager = Manager::new().await?;
    debug!("Created bluetooth manager");

    let adapters = manager.adapters().await?;
    debug!("Created bluetooth adapter");

    b_state.central = adapters.first().cloned();
    debug!("Attempting to start scan");

    debug!("Scan started");

    let central = b_state.central.as_mut();
    let mut events = Option::expect(central, "Did not found btleplug adapter")
        .events()
        .await?;
    while let Some(event) = events.next().await {
        debug!(
            "Some bluetooth event has occured. Event information{:?} ",
            event
        );
        if let CentralEvent::DeviceDiscovered(id) = event {
            match sender.send(id).await {
                Ok(_) => {
                    debug!("Device detected. Packet sent");
                }
                Err(e) => {
                    error!("Unable to send packet {}", e);
                }
            }
        }
    }

    Ok(())
}
