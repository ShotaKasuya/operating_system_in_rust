mod capability_registers;
mod operational_registers;

use core::marker::PhantomData;
use core::ptr;
pub use capability_registers::CapabilityRegisters;
pub use operational_registers::OperationalRegisters;

pub struct Mmio<T> {
    base_address: usize,
    phantom: PhantomData<T>,
}

impl<T> Mmio<T> {
    pub fn new(addr: usize) -> Self {
        Self {
            base_address: addr,
            phantom: PhantomData::default(),
        }
    }
    pub fn registers(&self) -> &T {
        unsafe {
            &*(self.base_address as *const T)
        }
    }

    pub fn registers_mut(&self) -> &mut T {
        unsafe {
            &mut *(self.base_address as *mut T)
        }
    }
}

trait VolatileRead {
    fn get_data(&self) -> &u32;

    fn read(&self) -> u32 {
        unsafe { ptr::read_volatile(self.get_data()) }
    }

    fn read_bit(&self, position: usize) -> bool {
        let mask: u32 = 1 << position;
        (self.read() & mask) != 0
    }
}

trait VolatileWrite:VolatileRead {
    fn get_data_mut(&mut self) -> &mut u32;

    fn write(&mut self, data: u32) {
        unsafe { ptr::write_volatile(self.get_data_mut(), data) }
    }
    
    fn write_bit(&mut self, on_off: bool, position: usize) {
        let mask: u32 = 1 << position;
        if on_off {
            unsafe { ptr::write_volatile(self.get_data_mut(), mask | self.read()) }
        } else {
            unsafe { ptr::write_volatile(self.get_data_mut(), (!mask) & self.read()) }
        }
    }
}


#[repr(C, packed)]
pub struct PortRegisters {
    portsc: u32, // Port Status and Control
    portpmsc: u32, // Port Power Management Status and Control
    portli: u32, // Port Link Info
    // reserved: ?
}

#[repr(C, packed)]
pub struct RuntimeRegisters {}

