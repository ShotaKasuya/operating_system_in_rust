use crate::usb::xhci::registers::{VolatileRead, VolatileWrite};
use crate::usb::xhci::registers::operational_registers::command_ring_control_register::CommandRingControlRegister;

pub mod command_ring_control_register;
mod device_context_base_array_pointer;

/// xHCI 規格書 5.4参照
#[repr(C, align(32))]
pub struct OperationalRegisters {
    usbcmd: USBCommandRegister, // USB Command
    usbsts: USBStatusRegister,  // USB Status
    pagesize: PageSizeRegister,              // Page Size
    reserved1: [u32; 2],
    dnctrl: DeviceNotificationControlRegister, // Device Notification Control
    crcr: CommandRingControlRegister,    // Command Ring Control
    reserved2: [u8; 0x10],
    dcbaap: u64, // Device Context Base Address Array Pointer
    config: u32, // Configure
}

#[repr(C, align(32))]
struct USBCommandRegister {
    data: u32,
}

const RUN_STOP: usize = 0;
const HOST_CONTROLLER_RESET: usize = 1;
const INTERRUPTER_ENABLE: usize = 2;
const HOST_SYSTEM_ERROR_ENABLE: usize = 3;
const LIGHT_HOST_CONTROLLER_RESET: usize = 7;
const CONTROLLER_SAVE_STATE: usize = 8;
const CONTROLLER_RESTORE_STATE: usize = 9;
const ENABLE_WRAP_EVENT: usize = 10;
const ENABLE_U3_MFINDEX_STOP: usize = 11;
const CEM_ENABLE: usize = 13;
const EXTENDED_TBC_ENABLE: usize = 14;
const EXTENDED_TBC_TRB_STATUS_ENABLE: usize = 15;
const VTIO_ENABLE: usize = 16;

impl VolatileRead for USBCommandRegister {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}

impl VolatileWrite for USBCommandRegister {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl USBCommandRegister {
    fn r_run_stop(&self) -> bool {
        self.read_bit(RUN_STOP)
    }
    fn w_run_stop(&mut self, bit: bool) {
        self.write_bit(bit, RUN_STOP);
    }
    fn r_host_controller_reset(&self) -> bool {
        self.read_bit(HOST_CONTROLLER_RESET)
    }
    fn w_host_controller_reset(&mut self, bit: bool) {
        self.write_bit(bit, HOST_CONTROLLER_RESET);
    }
    fn r_interrupt_enable(&self) -> bool {
        self.read_bit(INTERRUPTER_ENABLE)
    }
    fn w_interrupt_enable(&mut self, bit: bool) {
        self.write_bit(bit, INTERRUPTER_ENABLE);
    }
    fn r_host_system_error_enable(&self) -> bool {
        self.read_bit(HOST_SYSTEM_ERROR_ENABLE)
    }
    fn w_host_system_error_enable(&mut self, bit: bool) {
        self.write_bit(bit, HOST_SYSTEM_ERROR_ENABLE);
    }
    fn r_light_host_controller_reset(&self) -> bool {
        self.read_bit(LIGHT_HOST_CONTROLLER_RESET)
    }
    fn w_light_host_controller_reset(&mut self, bit: bool) {
        self.write_bit(bit, LIGHT_HOST_CONTROLLER_RESET);
    }
    fn r_controller_save_state(&self) -> bool {
        self.read_bit(CONTROLLER_SAVE_STATE)
    }
    fn w_controller_save_state(&mut self, bit: bool) {
        self.write_bit(bit, CONTROLLER_SAVE_STATE);
    }

    fn r_controller_restore_state(&self) -> bool {
        self.read_bit(CONTROLLER_RESTORE_STATE)
    }
    fn w_controller_restore_state(&mut self, bit: bool) {
        self.write_bit(bit, CONTROLLER_RESTORE_STATE);
    }
    fn r_enable_u3_mfindex_stop(&self) -> bool {
        self.read_bit(ENABLE_U3_MFINDEX_STOP)
    }
    fn w_enable_u3_mfindex_stop(&mut self, bit: bool) {
        self.write_bit(bit, ENABLE_U3_MFINDEX_STOP);
    }
    fn r_cem_enable(&self) -> bool {
        self.read_bit(CEM_ENABLE)
    }
    fn w_cem_enable(&mut self, bit: bool) {
        self.write_bit(bit, CEM_ENABLE);
    }
    fn r_extended_tbc_enable(&self) -> bool {
        self.read_bit(EXTENDED_TBC_ENABLE)
    }
    fn w_extended_tbc_enable(&mut self, bit: bool) {
        self.write_bit(bit, EXTENDED_TBC_ENABLE);
    }
    fn r_extended_tbc_trb_status_enable(&self) -> bool {
        self.read_bit(EXTENDED_TBC_TRB_STATUS_ENABLE)
    }
    fn w_extended_tbc_trb_status_enable(&mut self, bit: bool) {
        self.write_bit(bit, EXTENDED_TBC_TRB_STATUS_ENABLE);
    }

    fn r_vtio_enable(&self) -> bool {
        self.read_bit(VTIO_ENABLE)
    }
    fn w_vtio_enable(&mut self, bit: bool) {
        self.write_bit(bit, VTIO_ENABLE);
    }
}

#[repr(C, align(32))]
struct USBStatusRegister {
    data: u32,
}

const HCH_HALTED: usize = 0;
const HOST_SYSTEM_ERROR: usize = 2;
const EVENT_INTERRUPT: usize = 3;
const PORT_CHANGE_DETECT: usize = 4;
const SAVE_STATE_STATUS: usize = 8;
const RESTORE_STATE_STATUS: usize = 9;
const SAVE_RESTORE_ERROR: usize = 10;
const CONTROLLER_NOT_READY: usize = 11;
const HOST_CONTROLLER_ERROR: usize = 12;

impl VolatileRead for USBStatusRegister {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}

impl VolatileWrite for USBStatusRegister {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}

impl USBStatusRegister {
    fn r_hch_halted(&self) -> bool {
        self.read_bit(HCH_HALTED)
    }
    fn w_hch_halted(&mut self, bit: bool) {
        self.write_bit(bit, HCH_HALTED);
    }
    fn r_host_system_error(&self) -> bool {
        self.read_bit(HOST_SYSTEM_ERROR)
    }
    fn w_host_system_error(&mut self, bit: bool) {
        self.write_bit(bit, HOST_SYSTEM_ERROR);
    }
    fn r_event_interrupt(&self) -> bool {
        self.read_bit(EVENT_INTERRUPT)
    }
    fn w_event_interrupt(&mut self, bit: bool) {
        self.write_bit(bit, EVENT_INTERRUPT);
    }
    fn r_port_change_detect(&self) -> bool {
        self.read_bit(PORT_CHANGE_DETECT)
    }
    fn w_port_change_detect(&mut self, bit: bool) {
        self.write_bit(bit, PORT_CHANGE_DETECT);
    }
    fn r_save_state_status(&self) -> bool {
        self.read_bit(SAVE_STATE_STATUS)
    }
    fn w_save_state_status(&mut self, bit: bool) {
        self.write_bit(bit, SAVE_STATE_STATUS);
    }
    fn r_save_restore_error(&self) -> bool {
        self.read_bit(SAVE_RESTORE_ERROR)
    }
    fn w_save_restore_error(&mut self, bit: bool) {
        self.write_bit(bit, SAVE_RESTORE_ERROR);
    }

    fn r_controller_not_ready(&self) -> bool {
        self.read_bit(CONTROLLER_NOT_READY)
    }
    fn w_controller_not_ready(&mut self, bit: bool) {
        self.write_bit(bit, CONTROLLER_NOT_READY);
    }
    fn r_host_controller_error(&self) -> bool {
        self.read_bit(HOST_CONTROLLER_ERROR)
    }
    fn w_host_controller_error(&mut self, bit: bool) {
        self.write_bit(bit, HOST_CONTROLLER_ERROR);
    }
}

#[repr(C, align(32))]
struct PageSizeRegister {
    data: u32,
}

impl VolatileRead for PageSizeRegister {
    fn get_data(&self) -> &u32 {
        &self.data
    }
}

impl PageSizeRegister {
    fn page_size(&self) -> u16 {
        (self.read() & 0xFFFF) as u16
    }
}

#[repr(C, align(32))]
struct DeviceNotificationControlRegister {
    data: u32,
}
impl VolatileRead for DeviceNotificationControlRegister{
    fn get_data(&self) -> &u32 {
        &self.data
    }
}

impl VolatileWrite for DeviceNotificationControlRegister {
    fn get_data_mut(&mut self) -> &mut u32 {
        &mut self.data
    }
}
