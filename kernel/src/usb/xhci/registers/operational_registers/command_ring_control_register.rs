use crate::usb::xhci::registers::{VolatileRead, VolatileWrite};

#[repr(C, align(32))]
pub struct CommandRingControlRegister {
    high_bit: CRCRHigh,
    low_bit: CRCRLow,
}

pub struct CRCRHigh {
    data: u32,
}

pub struct CRCRLow {
    data: u32,
}

impl VolatileRead for CRCRHigh {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}
impl VolatileWrite for CRCRHigh {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl VolatileRead for CRCRLow {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}
impl VolatileWrite for CRCRLow {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}
const RING_CYCLE_STATE: usize = 0;
const COMMAND_STOP: usize = 1;
const COMMAND_ABORT: usize = 2;
const COMMAND_RING_RUNNING: usize = 3;

const COMMAND_RING_POINTER_OFFSET: usize = 6;

impl CommandRingControlRegister {
    pub fn r_ring_cycle_state(&self) -> bool {
        self.high_bit.read_bit(RING_CYCLE_STATE)
    }
    pub fn w_ring_cycle_state(&mut self, bit: bool) {
        self.high_bit.write_bit(bit, RING_CYCLE_STATE);
    }
    pub fn r_command_stop(&self) -> bool {
        self.high_bit.read_bit(COMMAND_STOP)
    }
    pub fn w_command_stop(&mut self, bit: bool) {
        self.high_bit.write_bit(bit, COMMAND_STOP);
    }
    pub fn r_command_abort(&mut self) -> bool {
        self.high_bit.read_bit(COMMAND_ABORT)
    }
    pub fn w_command_abort(&mut self, bit: bool) {
        self.high_bit.write_bit(bit, COMMAND_ABORT);
    }
    pub fn r_command_ring_running(&self) -> bool {
        self.high_bit.read_bit(COMMAND_RING_RUNNING)
    }
    pub fn r_command_ring_pointer(&self) -> u64 {
        let command_ring_pointer_lo = (self.high_bit.read() >> COMMAND_RING_POINTER_OFFSET) as u64;
        let command_ring_pointer_hi = self.low_bit.read() as u64;

        command_ring_pointer_lo | (command_ring_pointer_hi << (32 - COMMAND_RING_POINTER_OFFSET))
    }

    pub fn w_command_ring_pointer(&mut self, pointer: u64) {
        let before_pointer_lo = self.high_bit.read() & 0x0000_003F_u32;
        let pointer_lo = (pointer << COMMAND_RING_POINTER_OFFSET) as u32;
        let pointer_hi = (pointer >> 32) as u32;

        self.low_bit.write(before_pointer_lo | pointer_lo);
        self.high_bit.write(pointer_hi);
    }
}
