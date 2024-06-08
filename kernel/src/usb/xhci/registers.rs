mod capability_registers;
mod operational_registers;

use core::marker::PhantomData;
pub use capability_registers::CapabilityRegisters;
pub use operational_registers::OperationalRegisters;

struct Mmio<T> {
    base_address: usize,
    phantom: PhantomData<T>,
}

impl<T> Mmio<T> {
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

#[repr(C, packed)]
pub struct PortRegisters {
    portsc: u32, // Port Status and Control
    portpmsc: u32, // Port Power Management Status and Control
    portli: u32, // Port Link Info
    // reserved: ?
}

#[repr(C, packed)]
pub struct RuntimeRegisters {}

