use crate::usb::{calc_bar_address, make_address, read_class_code, read_data, read_vendor_id, write_address};

#[derive(Default, Debug)]
pub struct Device {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub header_type: u8,
    pub class_code: ClassCode,
}

#[derive(Default, Debug)]
pub struct ClassCode {
    base: u8,
    sub: u8,
    interface: u8,
}

impl Device {
    pub fn new(bus: u8, device: u8, function: u8, header_type: u8) -> Self {
        let class_code = read_class_code(bus, device, function);
        Self {
            bus,
            device,
            function,
            header_type,
            class_code: ClassCode::new(class_code),
        }
    }

    /// if return `None` your index out of range
    pub fn read_bar(&self, bar_index: usize) -> Option<u64> {
        if bar_index >= 6 {
            return None;
        }
        let addr = calc_bar_address(bar_index);
        let bar = self.read_conf_reg(addr);

        // 32bit address
        if (bar & 0b0100) == 0 {
            return Some(bar as u64);
        }

        // 64bit address
        if bar_index >= 0b0101 {
            return None;
        }

        let bar_upper = self.read_conf_reg(addr + 4) as u64;
        Some((bar as u64) | bar_upper << 32)
    }

    pub fn read_vendor_id(&self) -> u32 {
        read_vendor_id(self.bus, self.device, self.function)
    }
    pub fn read_conf_reg(&self, reg_addr: u8) -> u32 {
        write_address(make_address(self.bus, self.device, self.function, reg_addr));
        read_data()
    }
}

impl ClassCode {
    fn new(class_code: u32) -> Self {
        Self {
            base: ((class_code >> 24) & 0xFF) as u8,
            sub: ((class_code >> 16) & 0xFF) as u8,
            interface: ((class_code >> 8) & 0xFF) as u8,
        }
    }

    pub fn equal_b(&self, base: u8) -> bool {
        self.base == base
    }
    pub fn equal_bs(&self, base: u8, sub: u8) -> bool {
        self.equal_b(base) && self.sub == sub
    }
    pub fn equal_bsi(&self, base: u8, sub: u8, interface: u8) -> bool {
        self.equal_bs(base, sub) && self.interface == interface
    }
}