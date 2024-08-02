//! Host Controller Runtime Registers

use core::marker::PhantomData;
use accessor::marker::{AccessorTypeSpecifier, Readable, ReadOnly, ReadWrite};
use accessor::{single, Mapper};
use crate::registers::capability::RuntimeRegisterSpaceOffset;

/// Runtime Registers
///
/// この構造体には割り込みレジスタセットが含まれていないことに注意してください
/// [`InterruptRegisterSet`]を参照してください
#[derive(Debug)]
pub struct Runtime<M>
where
    M: Mapper,
{
    /// Microframe Index Register
    pub mfindex: single::ReadWrite<MicroframeIndexRegister, M>,
}

impl<M> Runtime<M>
where M: Mapper,
{
    /// 新しいHost Controller Runtime Registerへのアクセッサを作ります
    /// # Safety
    /// 呼び出し元はHost Controller Runtime Registerがこの構造体を通してのみアクセスされることを保証しなければならない
    /// # Panics
    /// このメソッドは`mmio_base`が正しくアラインメントされていないとパニックします
    pub unsafe fn new(mmio_base: usize, rtoff: RuntimeRegisterSpaceOffset, mapper: M)->Self{
        let base = mmio_base+usize::try_from(rtoff.get()).unwrap();

        Self {
            mfindex: single::ReadWrite::new(base, mapper),
        }
    }
}

/// Microframe Index Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MicroframeIndexRegister(u32);
impl MicroframeIndexRegister {
    ro_field!(0..=13, microframe_index, "Microframe Index", u16);
}
impl_debug_from_methods! {
    MicroframeIndexRegister {
        microframe_index,
    }
}

/// Interrupter Register Set
#[repr(C)]
#[derive(Debug)]
pub struct InterrupterRegisterSet<M>
where
    M: Mapper + Clone,
{
    base: usize,
    mapper: M,
}
impl <M> InterrupterRegisterSet<M>
where
M: Mapper+ Clone,
{
    /// Interrupter Register Setへの新しいアクセッサを作ります
    /// # Safety
    /// 呼び出し元はランタイムレジスタがこの構造体を通してのみアクセスされることを保証する必要があります
    /// # Panics
    /// Interrupter Register Setのベースアドレスが正しくアラインメントされていないとパニックします
    pub unsafe fn new(mmio_base: usize, rtoff: RuntimeRegisterSpaceOffset, mapper: M)-> Self{
        let base = mmio_base+usize::try_from(rtoff.get()).unwrap() + 0x20;
        assert_eq!(base % 0x20, 0, "base is not aligned");

        Self {base, mapper}
    }

    /// interrupterへのハンドラを返す
    /// # Panics
    /// このメソッドは`index> 1023`でパニックします
    pub fn interrupter(&self, index: usize) -> Interrupter<'_, M, ReadOnly> {
        unsafe {
            Interrupter::new(self.base, index, self.mapper.clone())
        }
    }

    /// interrupterへの新しい可変ハンドラを返す
    /// # Panics
    /// このメソッドは`index > 1023`でパニックする
    pub fn interrupter_mut(&mut self, index: usize)-> Interrupter<'_, M, ReadWrite> {
        unsafe {
            Interrupter::new(self.base, index, self.mapper.clone())
        }
    }
}

/// Interrupter
#[derive(Debug)]
pub struct Interrupter<'a, M, A>
where
    M: Mapper + Clone,
    A: AccessorTypeSpecifier + Readable, {
    /// Interrupter Management Register
    pub iman: single::Generic<InterrupterManagementRegister, M, A>,
    /// Interrupter Moderation Register
    pub imod : single::Generic<InterrupterModerationRegister, M, A>,
    /// Event Ring Segment Table Size Register
    pub erstsz: single::Generic<EventRingSegmentTableSizeRegister, M, A>,
    /// Event Ring Segment Table Base Address Register
    pub erstba: single::Generic<EventRingSegmentTableBaseAddressRegister, M, A>,
    /// Event Ring Dequeue Pointer Register
    pub erdp: single::Generic<EventRingDequeuePointerRegister, M, A>,
    // この割り込みの有効期間を親のInterrupterRegisterSetに結びつける
    // これにより複数の可変ハンドラが作成されるのを防ぐ
    _maker: PhantomData<&'a InterrupterRegisterSet<M>>,
}
impl <M,A> Interrupter<'_, M,A>
where
M: Mapper+Clone,
A:AccessorTypeSpecifier + Readable,
{
    /// interrupterへの新しいアクセッサを作る
    /// # Safety
    /// このInterrupterへの可変ハンドラは単一である必要があります
    /// # Panics
    /// このメソッドは`index > 1023`でパニックします
    unsafe fn new(interrupter_register_set_base: usize, index: usize, mapper: M)-> Self {
        assert!(index < 1024, "index out of range");
        let base = interrupter_register_set_base + index * 0x20;
        Self {
            iman: single::Generic::new(base, mapper.clone()),
            imod: single::Generic::new(base+0x04, mapper.clone()),
            erstsz: single::Generic::new(base+ 0x08, mapper.clone()),
            erstba: single::Generic::new(base+ 0x10, mapper.clone()),
            erdp: single::Generic::new(base+0x18, mapper),
            _maker: PhantomData,
        }
    }
}

/// Interrupter Management Register
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct InterrupterManagementRegister(u32);

impl InterrupterManagementRegister {
    rw1c_bit!(0, interrupt_pending, "Interrupt Pending");
    rw_bit!(1, interrupt_enable, "Interrupt Enable");
}
impl_debug_from_methods! {
    InterrupterManagementRegister {
        interrupt_pending,
        interrupt_enable
    }
}

/// Interrupter Moderation Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct InterrupterModerationRegister(u32);

impl InterrupterModerationRegister {
    rw_field!(
        0..=15,
        interrupt_moderation_intercal,
        "Interrupt Moderation Interval",
        u16
    );
    rw_field!(
        16..=31,
        interrupt_moderation_counter,
        "Interrupt Moderation Counter",
        u16
    );
}
impl_debug_from_methods! {
    InterrupterModerationRegister {
        interrupt_moderation_intercal,
        interrupt_moderation_counter,
    }
}

/// Event Ring Segment Table Size Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EventRingSegmentTableSizeRegister(u32);

impl EventRingSegmentTableSizeRegister {
    /// Event Ring Segment Tableがサポートするセグメント数を返す
    #[must_use]
    pub fn get(self) -> u16 {
        self.0.try_into().unwrap()
    }

    /// Event Ring Segment Tableがサポートするセグメント数セットする
    pub fn set(&mut self, s: u16) {
        self.0 = s.into();
    }
}

/// Event Ring Segment Table Base Address Register
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct EventRingSegmentTableBaseAddressRegister(u64);
impl EventRingSegmentTableBaseAddressRegister {
    /// Event Ring Segment Tableのベースアドレスを返します
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// Event Ring Segment Tableのベースアドレスをセットします
    /// # Panics
    /// このメソッドは64byteアラインされていない時パニックします
    pub fn set(&mut self, a: u64) {
        assert!(
            a.trailing_zeros() >= 6,
            "The Event Ring Segment Table Base Address must be 64-byte aligned"
        );
        self.0 = a;
    }
}

/// Event Ring Dequeue Pointer Register
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct EventRingDequeuePointerRegister(u64);
impl EventRingDequeuePointerRegister {
    rw_field!(
        0..=2,
        dequeue_erst_segment_index,
        "Dequeue ERST Segment Index",
        u8
    );
    rw1c_bit!(3, event_handler_busy, "Event Handler Busy");

    /// 現在のEvent Ring Dequeue Pointerのアドレスを返します
    #[must_use]
    pub fn event_ring_dequeue_pointer(self) -> u64 {
        self.0 & !0b1111
    }

    /// Event Ring Dequeue Pointerのアドレスをセットします
    /// # Panics
    /// このメソッドは16byteアラインされていないときパニックします
    pub fn set_event_ring_dequeue_pointer(&mut self, p: u64) {
        assert!(
            p.trailing_zeros() >= 4,
            "The Event Ring Dequeue Pointerは16byteアラインされなければいけません"
        );
        self.0 = p;
    }
}
impl_debug_from_methods! {
    EventRingDequeuePointerRegister {
        dequeue_erst_segment_index,
        event_handler_busy,
        event_ring_dequeue_pointer,
    }
}
