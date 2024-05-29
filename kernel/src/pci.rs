use core::arch::asm;

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

fn io_out32(addr: u16, data: u32) {
    unsafe {
        asm!(
        "mov dx, {}"
        "mov eax, {}"
        "out dx, eax"
        "ret"
        )
    }
}

fn io_in32(addr: u16) -> u32 {
    unsafe {
        asm!(
        "mov dx, {}"
        "in eax, {}"
        "ret"
        )
    }
}