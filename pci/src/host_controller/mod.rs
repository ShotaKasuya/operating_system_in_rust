use log::debug;
use xhci::accessor::Mapper;
use xhci::registers::capability::CapabilityParameters1;
use xhci::{ExtendedCapability, Registers};

pub struct HostController<M>
where
    M: Mapper + Clone,
{
    registers: Registers<M>,
}

impl<M> HostController<M>
where
    M: Mapper + Clone,
{
    pub fn new(mmio_base: usize, mapper: &M) -> Self {
        let mut registers = unsafe { Registers::new(mmio_base, mapper.clone()) };

        Self::request_host_controller_ownership(
            mmio_base,
            registers.capability.hccparams1.read_volatile(),
            mapper,
        );

        let mut usbcmd = registers.operational.usbcmd.read_volatile();
        usbcmd.clear_interrupter_enable();
        usbcmd.clear_host_system_error_enable();
        usbcmd.clear_enable_wrap_event();
        // ホストコントローラーはリセットする前に停止する必要があります
        if !registers.operational.usbsts.read_volatile().hc_halted() {
            usbcmd.clear_run_stop();
        }
        registers.operational.usbcmd.write_volatile(usbcmd);
        while !registers.operational.usbsts.read_volatile().hc_halted() {}

        // Reset Controller
        let mut usbcmd = registers.operational.usbcmd.read_volatile();
        usbcmd.set_host_controller_reset();
        registers.operational.usbcmd.write_volatile(usbcmd);

        debug!("xhci::HostController Initialize: waiting 1ms...\n");

        Self { registers }
    }

    ///
    fn request_host_controller_ownership(
        mmio_base: usize,
        hccparams1: CapabilityParameters1,
        mapper: &M,
    ) {
        let mut extended_capabilities = unsafe {
            xhci::extended_capabilities::List::new(mmio_base, hccparams1, mapper.clone())
        }
        .expect("The xHC does not support the xHCI Extended Capability.");

        for extended_capability in &mut extended_capabilities {
            match extended_capability {
                Ok(extended_capability) => match extended_capability {
                    ExtendedCapability::UsbLegacySupport(mut usb) => {
                        let mut semaphore = usb.usblegsup.read_volatile();
                        if semaphore.hc_os_owned_semaphore() {
                            return;
                        }
                        semaphore.set_hc_os_owned_semaphore();
                        usb.usblegsup.write_volatile(semaphore);
                        debug!("waiting until OS owns xHC...\n");
                        loop {
                            let leg_sup = usb.usblegsup.read_volatile();
                            if leg_sup.hc_bios_owned_semaphore() && !leg_sup.hc_os_owned_semaphore()
                            {
                                break;
                            }
                        }
                    }
                    _ => {}
                },
                Err(err) => {}
            }
        }
    }
}
