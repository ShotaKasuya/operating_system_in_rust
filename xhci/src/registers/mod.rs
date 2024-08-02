use accessor::{array, Mapper};
use crate::registers::capability::Capability;
use crate::registers::doorbell::Doorbell;
use crate::registers::operational::{Operational, PortRegisterSet};
use crate::registers::runtime::{InterrupterRegisterSet, Runtime};

pub mod capability;
mod doorbell;
mod operational;
mod runtime;


#[derive(Debug)]
pub struct Registers<M>
where M:Mapper+Clone
{
    /// Host Controller Capability Register
    pub capability: Capability<M>,
    /// Doorbell Array
    pub doorbell: array::ReadWrite<Doorbell, M>,
    /// Host Controller Operational Register
    pub operational:Operational<M>,
    /// Port Register Set Array
    pub port_register_set: array::ReadWrite<PortRegisterSet, M>,
    /// Runtime Registers
    pub runtime: Runtime<M>,
    /// Interrupter Register Set Array
    pub interrupter_register_set: InterrupterRegisterSet<M>,
}
impl <M> Registers<M>
where M:Mapper+Clone,
{
    /// [`Registers`]の新しいインスタンスを作成します
    /// # Safety
    /// 呼び出し元はxHCIのレジスタにこの構造体を通してのみアクセスすることを保証しなければなりません
    /// # Panics
    /// このメソッドは`mmio_base`が正しくアラインメントされていない時パニックします
    pub unsafe fn new(mmio_base: usize, mapper: M)-> Self {
        let capability = Capability::new(mmio_base, &mapper);
        let doorbell = Doorbell::new(mmio_base, &capability, mapper.clone());
        let operational = Operational::new(mmio_base, capability.caplength.read_volatile(), &mapper);
        let port_register_set = PortRegisterSet::new(mmio_base, &capability, mapper.clone());
        let runtime = Runtime::new(mmio_base, capability.rtsoff.read_volatile(), mapper.clone());
        let interrupter_register_set = InterrupterRegisterSet::new(mmio_base, capability.rtsoff.read_volatile(), mapper);

        Self {
            capability,
            doorbell,
            operational,
            port_register_set,
            runtime,
            interrupter_register_set,
        }
    }
}