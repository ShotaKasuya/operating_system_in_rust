
use xhci::accessor::Mapper;
use xhci::{ExtendedCapability, Registers};
use xhci::registers::capability::CapabilityParameters1;

pub struct HostController<M>
where M: Mapper + Clone{
    registers: Registers<M>,
}

impl<M> HostController<M>
where M: Mapper + Clone
{
    pub fn new(mmio_base: usize, mapper: &M) -> Self {
        let mut registers = unsafe {
            Registers::new(mmio_base, mapper.clone())
        };

        Self::request_host_controller_ownership(mmio_base, registers.capability.hccparams1.read_volatile(), mapper);

        Self {
            registers
        }
    }

    ///
    fn request_host_controller_ownership(mmio_base: usize, hccparams1: CapabilityParameters1, mapper: &M) {
        let mut extended_capabilities = unsafe {
            xhci::extended_capabilities::List::new(mmio_base, hccparams1, mapper.clone())
        }.expect("The xHC does not support the xHCI Extended Capability.");

        for extended_capability in &mut extended_capabilities {
            match extended_capability {
                Ok(extended_capability) => {
                    match extended_capability {
                        xhci::extended_capabilities::ExtendedCapability::UsbLegacySupport(mut usb) => {
                            if usb.usblegsup.read_volatile().hc_os_owned_semaphore() {
                                return;
                            }
                            let mut semaphore = usb.usblegsup.read_volatile();
                            semaphore. set_hc_os_owned_semaphore();
                            usb.usblegsup.write_volatile(semaphore);
                        }
                        _ => {}
                    }
                }
                Err(err) => {

                }
            }
        }


    }
}