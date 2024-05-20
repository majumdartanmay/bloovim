use log::error;
use tokio::sync::mpsc::Sender;

use crate::BState;
use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::platform::PeripheralId;
use btleplug::Result;
use futures::StreamExt;
use log::debug;

pub async fn start_bluetooth_stream(sender: &Sender<PeripheralId>, b_state: &BState) -> Result<()> {
    let central = &b_state.central;
    let mut events = central.events().await?;
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
