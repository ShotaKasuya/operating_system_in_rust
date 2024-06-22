use crate::usb::xhci::registers::{VolatileRead, VolatileWrite};

pub struct ConfigureRegister{
    data:u32,
}


impl VolatileRead for ConfigureRegister{
    fn get_data(&self) -> &u32 {
        &self.data
    }
}
impl VolatileWrite for ConfigureRegister{
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl ConfigureRegister {
}