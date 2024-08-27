
use xhci::accessor::Mapper;
use xhci::Registers;
use xhci::registers::capability::CapabilityParameters1;

pub struct HostController<M> {
    registers: Registers<M>,
}

impl<M> HostController<M>
where M: Mapper
{
    pub fn new(mmio_base: usize, mapper: &M) -> Self {
        let mut registers = unsafe {
            Registers::new(mmio_base, mapper)
        };

        Self::request_host_controller_ownership(mmio_base, registers.capability.hccparams1.read_volatile(), mapper);

        Self {
            registers
        }
    }

    ///
    fn request_host_controller_ownership(mmio_base: usize, hccparams1: CapabilityParameters1, mapper: &M) {
        let extended_capabilities = unsafe {
            xhci::extended_capabilities::List::new(mmio_base, hccparams1, mapper)
        }.expect("The xHC does not support the xHCI Extended Capability.");

        for extended_capability in extended_capabilities {
            match extended_capability {
                Ok(extended_capability) => {
                    match extended_capability {
                        xhci::extended_capabilities::ExtendedCapability::UsbLegacySupport(usb) => {

                        }
                    }
                }
                Err(err) => {

                }
            }
        }


    }
}