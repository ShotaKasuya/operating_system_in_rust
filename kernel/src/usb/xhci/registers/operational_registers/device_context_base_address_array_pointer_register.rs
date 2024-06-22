use crate::usb::xhci::registers::{VolatileRead, VolatileWrite};

pub struct DeviceContextBaseAddressArrayPointerRegister {
    register_hi: DCBAAPHi,
    register_lo: DCBAAPLo,
}

pub struct DCBAAPHi {
    data: u32,
}

pub struct DCBAAPLo {
    data: u32,
}

impl VolatileRead for DCBAAPHi {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}

impl VolatileWrite for DCBAAPHi {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl VolatileRead for DCBAAPLo {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}

impl VolatileWrite for DCBAAPLo {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl DeviceContextBaseAddressArrayPointerRegister {
    pub fn r_device_context_base_address_array_pointer(&self)->u64{
        let pointer_hi=self.register_hi.read() as u64;
        let pointer_lo=self.register_lo.read() as u64;

        (pointer_hi << 32) | pointer_lo
    }
    
    pub fn w_device_context_base_address_array_pointer(&mut self, pointer:u64){
    }
}