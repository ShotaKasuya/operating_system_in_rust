use core::arch::asm;
use core::error;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref DEVICES:Mutex<Devices> = Mutex::new(Devices::default());
}
#[derive(Default, Debug)]
pub struct Devices {
    pub devices: [Device; 32],
    pub num: usize,
}

#[derive(Default, Debug)]
pub struct Device {
    bus: u8,
    device: u8,
    function: u8,
    header_type: u8,
}

pub enum PciError {
    Full
}

pub fn scan_all_bus() -> Result<(), PciError> {
    DEVICES.lock().clear();

    let header_type = read_header_type(0, 0, 0);
    if is_single_function_device(header_type) {
        scan_bus(0)?;
    }

    for function in 1..8 {
        if read_vendor_id(0, 0, function) == 0xFFFF {
            continue;
        }
        scan_bus(function)?;
    }
    Ok(())
}

fn scan_bus(bus: u8) -> Result<(), PciError> {
    for device in 0..32 {
        if read_vendor_id(bus, device, 0) == 0xFFFF {
            continue;
        }
        scan_device(bus, device)?;
    }
    Ok(())
}

fn scan_device(bus: u8, device: u8) -> Result<(), PciError> {
    scan_function(bus, device, 0)?;
    if is_single_function_device(read_header_type(bus, device, 0)) {
        return Ok(());
    }

    for function in 0..8 {
        if read_vendor_id(bus, device, function) == 0xFFFF {
            continue;
        }
        scan_function(bus, device, function)?;
    }
    Ok(())
}

fn scan_function(bus: u8, device: u8, function: u8) -> Result<(), PciError> {
    let header_type = read_header_type(bus, device, function);
    {
        DEVICES.lock().add_device(Device::new(bus, device, function, header_type))?;
    }
    let class_code = read_class_code(bus, device, function);
    let base = ((class_code >> 24) & 0xFF) as u8;
    let sub = ((class_code >> 16) & 0xFF) as u8;

    if base == 0x06 && sub == 0x04 {
        // standard PCI-PCI bridge
        let bus_numbers = read_bus_numbers(bus, device, function);
        let secondary_bus: u8 = ((bus_numbers >> 8) & 0xFF) as u8;

        return scan_bus(secondary_bus);
    }
    Ok(())
}

impl Devices {
    fn add_device(&mut self, device: Device) -> Result<(), PciError> {
        if self.is_full() {
            return Err(PciError::Full);
        }

        self.set_new_device(device);
        self.num += 1;
        Ok(())
    }
    fn clear(&mut self) {
        self.num = 0;
    }
    fn is_full(&self) -> bool {
        self.num == self.devices.len()
    }

    fn set_new_device(&mut self, device: Device) {
        self.devices[self.num] = device;
    }
}

impl Device {
    fn new(bus: u8, device: u8, function: u8, header_type: u8) -> Self {
        Self {
            bus,
            device,
            function,
            header_type,
        }
    }
}

// CONFIG_ADDRESSレジスタのIOポートアドレス
const K_CONFIG_ADDRESS: u16 = 0x0CF8;
// CONFIG_DATAレジスタ
const K_CONFIG_DATA: u16 = 0x0CFC;

fn make_address(bus: u8, device: u8, function: u8, reg_addr: u8) -> u32 {
    let shl = |x: u8, bits: usize| -> u32 {
        (x as u32) << bits
    };

    shl(1, 31) | shl(bus, 16) | shl(device, 11) | shl(function, 8) | (reg_addr & 0xFC) as u32
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

pub fn read_device_id(bus: u8, device: u8, function: u8) -> u16 {
    write_address(make_address(bus, device, function, 0x00));
    (read_data() >> 16) as u16
}

pub fn read_class_code(bus: u8, device: u8, function: u8) -> u32 {
    write_address(make_address(bus, device, function, 0x0C));
    read_data()
}

pub fn read_header_type(bus: u8, device: u8, function: u8) -> u8 {
    write_address(make_address(bus, device, function, 0x0C));
    ((read_data() >> 16) & 0xFF) as u8
}

pub fn read_vendor_id(bus: u8, device: u8, function: u8) -> u32 {
    write_address(make_address(bus, device, function, 0x00));
    read_data() & 0xFFFF
}

pub fn read_bus_numbers(bus: u8, device: u8, function: u8) -> u32 {
    write_address(make_address(bus, device, function, 0x18));
    read_data()
}

fn is_single_function_device(header_type: u8) -> bool {
    (header_type & 0x80) == 0
}

fn io_out32(addr: u16, data: u32) {
    unsafe {
        asm!("mov dx, {0:x}", in(reg) addr);
        asm!("mov eax, {0:e}", in(reg) data);
        asm!("out dx, eax");
    }
}

fn io_in32(addr: u16) -> u32 {
    let mut ret: u32;
    unsafe {
        asm!("mov dx, {0:x}", in(reg) addr);
        asm!("in {0:e}, dx", out(reg) ret);
    }
    ret
}

impl Debug for PciError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Display for PciError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.description())
    }
}

impl error::Error for PciError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl PciError {
    fn description(&self) -> &'static str {
        match self { PciError::Full => "PCI Device is Full" }
    }
}