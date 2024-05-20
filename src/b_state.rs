use btleplug::api::Manager as _;
use btleplug::platform::Adapter;
use btleplug::platform::Manager;
use log::debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct BState {
    pub central: Rc<Adapter>,
}

impl BState {
    pub async fn new<'a>() -> Self {
        let manager = Manager::new().await.expect("Unable to create manager");
        debug!("Created bluetooth manager");

        let adapters = manager.adapters().await.expect("Unable to create adapters");
        let adapter = adapters.first().unwrap();
        debug!("Created bluetooth adapter");
        Self {
            central: Rc::new(adapter.clone()),
        }
    }
}
