use core::ptr;
use crate::usb::xhci::registers::Mmio;

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
    pub hccparams2: HCCPARAMS2,      // Capability Parameters2
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
pub struct HCCPARAMS2 {
    data: u32,
}

impl HCCPARAMS2 {
    fn u3_entry_capability(&self) -> bool {
        (self.data & 0x01) != 0
    }
    fn configure_endpoint_command_max_exit_latency_too_large_capability(&self) -> bool {
        (self.data & 0x02) != 0
    }
    fn force_save_context_capability(&self) -> bool {
        (self.data & 0x04) != 0
    }
    fn compliance_transition_capability(&self) -> bool {
        (self.data & 0x08) != 0
    }
    fn large_esit_payload_capability(&self) -> bool {
        (self.data & 0x10) != 0
    }
    fn configuration_information_capability(&self) -> bool {
        (self.data & 0x20) != 0
    }
    fn extended_tbc_capability(&self) -> bool {
        (self.data & 0x40) != 0
    }
    fn extended_tbc_status_capability(&self) -> bool {
        (self.data & 0x80) != 0
    }
    fn extended_property_capability(&self) -> bool {
        (self.data & 0x0100) != 0
    }
    fn virtualization_based_trusted_io_capability(&self) -> bool {
        (self.data & 0x0200) != 0
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


/// FIXME: あたまわるい
impl Mmio<CapabilityRegisters> {
    /// cap_length
    pub fn read_cap_length(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().cap_length
            )
        }
    }
    /// cap_length
    pub fn read_hci_version(&self) -> u16 {
        unsafe {
            ptr::read_volatile(
                &self.registers()
                // &self.registers().hci_version
            ).hci_version
        }
    }
    /// hcsparams1
    pub fn read_hcs1_max_device_slots(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams1
            ).max_device_slots()
        }
    }
    pub fn read_hcs1_max_interrupters(&self) -> u16 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams1
            ).max_interrupters()
        }
    }
    pub fn read_max_ports(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams1
            ).max_ports()
        }
    }
    /// hcsparams2
    pub fn read_hcs2_isochronous(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams2
            ).isochronous_scheduling_threshold()
        }
    }
    pub fn read_hcs2_event_ring_segment_table_max(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams2
            ).event_ring_segment_table_max()
        }
    }
    pub fn read_hcs2_max_scratchpad_buffer_high(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams2
            ).max_scratchpad_buffers_high()
        }
    }
    pub fn read_hcs2_scratchpad_restore(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams2
            ).scratchpad_restore()
        }
    }
    pub fn read_hcs2_max_scratchpad_buffer_low(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams2
            ).max_scratchpad_buffers_low()
        }
    }
    /// hcsparams3
    pub fn read_hcs3_u1_device_exit_latency(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams3
            ).u1_device_exit_latency()
        }
    }
    pub fn read_hcs3_u2_device_exit_latency(&self) -> u16 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hcsparams3
            ).u2_device_exit_latency()
        }
    }
    /// hccparams1
    pub fn read_hcc_addressing_capability_64(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).addressing_capability_64()
        }
    }
    pub fn read_hcc_bw_negotiation_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).bw_negotiation_capability()
        }
    }
    pub fn read_hcc_context_size(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).context_size()
        }
    }
    pub fn read_hcc_port_power_control4(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).port_power_control()
        }
    }
    pub fn read_hcc_port_indicators(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).port_indicators()
        }
    }
    pub fn read_hcc_light_hc_reset_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).light_hc_reset_capability()
        }
    }
    pub fn read_hcc_latency_tolerance_messaging_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).latency_tolerance_messaging_capability()
        }
    }
    pub fn read_hcc_no_secondary_sid_support(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).no_secondary_sid_support()
        }
    }
    pub fn read_hcc_parse_all_event_data(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).parse_all_event_data()
        }
    }
    pub fn read_hcc_stopped_short_packet_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).stopped_short_packet_capability()
        }
    }
    pub fn read_hcc_stopped_edtla_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).stopped_edtla_capability()
        }
    }
    pub fn read_hcc_contiguous_frame_id_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).contiguous_frame_id_capability()
        }
    }
    pub fn read_hcc_maximum_primary_stream_array_size(&self) -> u8 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).maximum_primary_stream_array_size()
        }
    }
    pub fn read_hcc_xhci_extended_capabilites_pointer(&self) -> u16 {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams1
            ).xhci_extended_capabilites_pointer()
        }
    }
    pub fn read_dboff(&self) -> u32 {
        unsafe {
            ptr::read_volatile(
                &self.registers().dboff
            )
        }
    }
    pub fn read_rts_off(&self) -> u32 {
        unsafe {
            ptr::read_volatile(
                &self.registers().rtsoff
            )
        }
    }

    pub fn read_hcc2_u3_entry_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).u3_entry_capability()
        }
    }
    pub fn read_hcc2_configure_endpoint_command_max_exit_latency_too_large_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).configure_endpoint_command_max_exit_latency_too_large_capability()
        }
    }
    pub fn read_hcc2_force_save_context_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).force_save_context_capability()
        }
    }
    pub fn read_hcc2_compliance_transition_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).compliance_transition_capability()
        }
    }
    pub fn read_hcc2_large_esit_payload_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).large_esit_payload_capability()
        }
    }
    pub fn read_hcc2_configuration_information_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).configuration_information_capability()
        }
    }

    pub fn read_hcc2_extended_tbc_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).extended_tbc_capability()
        }
    }
    pub fn read_hcc2_extended_property_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).extended_property_capability()
        }
    }    pub fn read_hcc2_virtualization_based_trusted_io_capability(&self) -> bool {
        unsafe {
            ptr::read_volatile(
                &self.registers().hccparams2
            ).virtualization_based_trusted_io_capability()
        }
    }
}