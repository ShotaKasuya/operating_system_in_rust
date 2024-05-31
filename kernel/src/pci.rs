use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!{
    pub static ref DEVICES:Mutex<[Device;32]> = Mutex::new(Default::default());
}

#[derive(Default)]
pub struct Device {
    bus : u8,
    device: u8,
    function: u8,
    header_type:u8,
}

// CONFIG_ADDRESSレジスタのIOポートアドレス
const K_CONFIG_ADDRESS: u16 = 0x0CF8;
// CONFIG_DATAレジスタ
const K_CONFIG_DATA: u16 = 0x0CFC;

fn make_address(bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
    let shl = |x: u8, bits: u64| -> u32 {
        (x << bits) as u32
    };

    shl(1, 32) | shl(bus, 16) | shl(device, 11) | shl(function, 8) | (reg_addr & 0xFC) as u32
}

pub fn write_address(addr: u32) {
    io_out32(K_CONFIG_ADDRESS, addr);
}

pub fn write_data(value: u32) {
    io_out32(K_CONFIG_DATA, value);
}

pub fn read_data() -> u32 {
    io_in32(K_CONFIG_DATA)
}

pub fn read_vendor_id(bus: u8, device: u8, function:u8) -> u32 {
    write_address(make_address(bus, device, function, 0x00));
    read_data() & 0xFFFF
}

fn io_out32(addr: u16, data: u32) {
    unsafe {
        asm!("mov dx, {0:x}", in(reg) addr);
        asm!("mov eax, {0:e}", in(reg) data);
        asm!("out dx, eax");
    }
}

fn io_in32(addr: u16) -> u32 {
    let mut ret:u32;
    unsafe {
        asm!("mov dx, {0:x}", in(reg) addr);
        asm!("in {0:e}, dx", out(reg) ret);
    }
    ret
}