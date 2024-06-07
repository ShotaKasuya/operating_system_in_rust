use bootloader_api::info::PixelFormat::U8;
use volatile::Volatile;
use crate::usb::xhci::registers::{CapabilityRegisters, OperationalRegisters};

struct Controller {
    mmio_base: u64,
    cap_: Volatile<*const CapabilityRegisters>,
    op_: Volatile<*mut OperationalRegisters>,
    max_ports: u8,
}

impl Controller {
    pub fn new(mmio_base: u64) -> Self {
        let cap_ = mmio_base as *const CapabilityRegisters;
        let op_ = unsafe { (mmio_base + (cap_.read().cap_length) as u64) as *mut OperationalRegisters };
        let max_ports = unsafe { cap_.read().hcsparams1.max_ports() };
        
        Self {
            mmio_base,
            cap_: Volatile::new(cap_),
            op_: Volatile::new(op_),
            max_ports,
        }
    }
}