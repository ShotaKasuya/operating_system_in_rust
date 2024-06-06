
/// xHCI 規格書 5.4参照
#[repr(C, packed)]
pub struct OperationalRegisters {
    usbcmd: u32,                // USB Command
    usbsts: u32,                // USB Status
    pagesize:u32,               // Page Size
    reserved1:[u8; 8],
    dnctrl: u32,                // Device Notification Control
    crcr: u8,                   // Command Ring Control
    reserved2:[u8; 0x10],
    dcbaap: u64,                // Device Context Base Address Array Pointer
    config: u32,                // Configure
}

