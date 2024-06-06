
/// xHCI 規格書 5.3参照
#[repr(C, packed)]
pub struct CapabilityRegisters {
    pub cap_length: u8,             // Capability Register Length
    pub reserved: u8,               // Reserved
    pub hci_version: u16,           // Interface Version Number
    pub hcsparams1: HCSPARAMS1,     // Structural Parameters1
    pub hcsparams2: HCSPARAMS2,     // Structural Parameters1
    pub hcsparams3: HCSPARAMS3,     // Structural Parameters1
    pub hccparams1: HCCPARAMS1,     // Capability Parameters
    pub dboff: u32,                 // Doorbell Offset
    pub rtsoff: u32,                // Runtime Registers Space Offset
    pub hccparams2: HCCPARAMS1,      // Capability Parameters2
}
#[repr(C, packed)]
pub struct HCSPARAMS1 {
    data: u32,
}

impl HCSPARAMS1 {
    pub fn max_device_slots(&self) -> u8 {
        (self.data & 0x0000_00FF) as u8
    }
    pub fn max_interrupters(&self) -> u16 {
        ((self.data & 0x0007_FF00) >> 8) as u16
    }
    pub fn max_ports(&self) -> u8 {
        ((self.data & 0xFF00_0000) >> 24) as u8
    }
}

#[repr(C, packed)]
pub struct HCSPARAMS2 {
    data: u32,
}

impl HCSPARAMS2 {
    pub fn isochronous_scheduling_threshold(&self) -> u8 {
        (self.data & 0x0000_000F) as u8
    }
    pub fn event_ring_segment_table_max(&self) -> u8 {
        ((self.data & 0x0000_00F0) >> 4) as u8
    }
    pub fn max_scratchpad_buffers_high(&self) -> u8 {
        ((self.data & 0x03E0_0000) >> 21) as u8
    }
    pub fn scratchpad_restore(&self) -> bool {
        (self.data & 0x0400_0000) != 0
    }
    pub fn max_scratchpad_buffers_low(&self) -> u8 {
        ((self.data & 0xF800_0000) >> 27) as u8
    }
}

#[repr(C, packed)]
pub struct HCSPARAMS3 {
    data: u32,
}

impl HCSPARAMS3 {
    pub fn u1_device_exit_latency(&self) -> u8 {
        (self.data & 0xFF) as u8
    }
    pub fn u2_device_exit_latency(&self) -> u16 {
        ((self.data & 0x00FF_0000) >> 16) as u16
    }
}


#[repr(C, packed)]
pub struct HCCPARAMS1 {
    data: u32,
}

impl HCCPARAMS1 {
    pub fn addressing_capability_64(&self) -> bool {
        (self.data & 0x0001) != 0
    }
    pub fn bw_negotiation_capability(&self) -> bool {
        (self.data & 0x0002) != 0
    }
    pub fn context_size(&self) -> bool {
        (self.data & 0x0004) != 0
    }
    pub fn port_power_control(&self) -> bool {
        (self.data & 0x0008) != 0
    }
    pub fn port_indicators(&self) -> bool {
        (self.data & 0x0010) != 0
    }
    pub fn light_hc_reset_capability(&self) -> bool {
        (self.data & 0x0020) != 0
    }
    pub fn latency_tolerance_messaging_capability(&self) -> bool {
        (self.data & 0x0040) != 0
    }
    pub fn no_secondary_sid_support(&self) -> bool {
        (self.data & 0x0080) != 0
    }
    pub fn parse_all_event_data(&self) -> bool {
        (self.data & 0x0100) != 0
    }
    pub fn stopped_short_packet_capability(&self) -> bool {
        (self.data & 0x0200) != 0
    }
    pub fn stopped_edtla_capability(&self) -> bool {
        (self.data & 0x0400) != 0
    }
    pub fn contiguous_frame_id_capability(&self) -> bool {
        (self.data & 0x0800) != 0
    }
    pub fn maximum_primary_stream_array_size(&self) -> u8 {
        ((self.data & 0xF000) >> 12) as u8
    }
    pub fn xhci_extended_capabilites_pointer(&self) -> u16 {
        ((self.data & 0xFFFF_0000) >> 16) as u16
    }
}

#[repr(C, packed)]
pub struct DBOFF {
    data: u32,
}

impl DBOFF {
    pub fn doorbell_array_offset(&self) -> u32 {
        self.data & 0x3FFF_FFFF
    }
}
