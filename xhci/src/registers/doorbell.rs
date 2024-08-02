//! Doorbell Register

use accessor::{array, Mapper};
use crate::registers::capability::Capability;

/// A type alias to [`Doorbell`] register for backward compability
#[deprecated="Use `Doorbell` instead of `Register`"]
pub type Register = Doorbell;

/// The element of the Doorbell Array
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct Doorbell(u32);
impl Doorbell {
    /// Doorbell Arrayへの新しいアクセッサを作る
    /// # Safety
    /// 呼び出し元はアクセッサが一度だけ作成されることを保証する必要があります
    /// # Panic
    /// 正しくアラインされていない場合パニックします
    pub unsafe fn new<M1, M2    >(
        mmio_base: usize,
        capability: &Capability<M2>,
        mapper: M1,
    )->array::ReadWrite<Self, M1>
    where
    M1: Mapper,
    M2: Mapper+Clone,
    {
        let base = mmio_base + usize::try_from(capability.dboff.read_volatile().get()).unwrap();
        array::ReadWrite::new(
            base,
            capability
                .hcsparams1
                .read_volatile()
                .number_of_device_slots()
                .into(),
            mapper,
        )
    }

    rw_field!(0..=7, doorbell_target, "Doorbell Target", u8);
    rw_field!(16..=31, doorbell_stream_id, "Doorbell Stream ID", u16);
}
impl_debug_from_methods! (Doorbell {
    doorbell_target,
    doorbell_stream_id,
});