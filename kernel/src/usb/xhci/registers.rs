mod capability_registers;
mod operational_registers;

pub use capability_registers::CapabilityRegisters;
pub use operational_registers::OperationalRegisters;

#[repr(C, packed)]
pub struct PortRegisters {
    portsc: u32, // Port Status and Control
    portpmsc: u32, // Port Power Management Status and Control
    portli: u32, // Port Link Info
    // reserved: ?
}

#[repr(C, packed)]
pub struct RuntimeRegisters {}

