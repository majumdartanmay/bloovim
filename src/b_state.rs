use btleplug::platform::Adapter;

pub struct BState {
    pub central: Option<Adapter>,
}
