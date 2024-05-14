use btleplug::platform::Adapter;

pub struct BState<'a> {
    pub central: Option<&'a Adapter>,
}
