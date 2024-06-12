use crate::usb::xhci::registers::{CapabilityRegisters, Mmio, OperationalRegisters};

struct Controller {
    mmio_base: u64,
    cap_: Mmio<CapabilityRegisters>,
    op_: Mmio<OperationalRegisters>,
    max_ports: u8,
}

impl Controller {
    pub fn new(mmio_base: u64) -> Self {
        let cap_ = Mmio::<CapabilityRegisters>::new(mmio_base as usize);
        let op_ = Mmio::<OperationalRegisters>::new((mmio_base + (cap_.read_cap_length() as u64)) as usize);
        let max_ports = cap_.read_max_ports();

        Self {
            mmio_base,
            cap_,
            op_,
            max_ports,
        }
    }
}