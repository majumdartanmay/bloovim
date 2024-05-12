use log::error;
use tokio::sync::mpsc::Sender;

use btleplug::api::Central;
use btleplug::api::CentralEvent;
use btleplug::api::ScanFilter;
use btleplug::platform::Manager;
use btleplug::platform::PeripheralId;
use btleplug::{api::Manager as _, Result};
use futures::StreamExt;
use log::debug;

pub async fn start_bluetooth_stream(sender: &Sender<PeripheralId>) -> Result<()> {
    debug!("Entered bluetooth scan zone");

    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.first().unwrap();

    debug!("Started scan");
    central.start_scan(ScanFilter::default()).await?;

    let mut events = central.events().await?;
    while let Some(event) = events.next().await {
        debug!("Some bluetooth event has occured {:?}", event);
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
