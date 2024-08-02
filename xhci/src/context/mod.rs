//! The xHCI Contexts
//!
//! 各Contextのサイズは実際のContextの大きさと同じです
//!
//! Contextのサイズに関係なくコンテキストへの参照を可能にするために、
//! すべてのコンテキストは、フィールドにアクセスして変更するためのメソッドを実装する
//! ハンドラー特性を実装します。
//! 使用可能なメソッドについては、各特性のドキュメントを参照してください


use core::fmt;
use core::fmt::Formatter;
use bit_field::BitField;

#[macro_use]
mod macros;

/// The number of Endpoint Contexts in a Device Context
pub const NUM_OF_ENDPOINT_CONTEXTS: usize = 31;

/// 32byte Input Context
pub type Input32Byte = Input<8>;



/// Input Context
///
/// Refer to [`InputHandler`] for the available methods
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Input<const N:usize> {
}

/// Input Control Context
/// Refer to [`InputControlHandler`] for the available methods
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InputControl<const N: usize>([u32; N]);
impl_constructor!(InputControl, "Input Control");
impl <const N:usize> InputControl<N> {
    const fn new() -> Self {
        Self([0; N])
    }
}
impl <const N:usize> AsRef<[u32]> for  InputControl<N> {
    fn as_ref(&self) -> &[u32] {
        &self.0
    }
}
impl <const N:usize> AsMut<[u32]> for  InputControl<N> {
    fn as_mut(&mut self) -> &mut [u32] {
        &mut self.0
    }
}
impl <const N:usize> InputControlHandler for  InputControl<N> {}

impl <const N:usize> fmt::Debug for  InputControl<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("InputControl")
            .field("Drop Context flags", &self.0[0])
            .field("Add Context flags", &self.0[1])
            .finish()
    }
}

/// A trait to handle Input Control Context
pub trait InputControlHandler: AsRef<[u32]> + AsMut<[u32]> {
    /// `i`番目のDrop Context flagを返す(0 index)
    /// # Panics
    /// このメソッドは`i < 2 || i > 31`でパニックします
    #[must_use]
    fn drop_context_flag(&self, i:usize) -> bool {
        self.ensure_drop_context_index_within_range(i);
        self.as_ref()[0].get_bit(i)
    }

    /// `i`番目のDrop Context flagに0をセットする(0 index)
    /// # Panics
    /// このメソッドは`i < 2 || i > 31`でパニックします
    fn set_drop_context_flag(&mut self, i:usize) {
        self.ensure_drop_context_index_within_range(i);
        self.as_mut()[0].set_bit(i, true);
    }

    /// `i`番目のDrop Context flagに1をセットする(0 index)
    /// # Panics
    /// このメソッドは`i < 2 || i > 31`でパニックします
    fn clear_drop_context_flag(&mut self, i:usize){
        self.ensure_drop_context_index_within_range(i);
        self.as_mut()[0].set_bit(i, false);
    }

    /// `i`番目のAdd Context flagを返す
    /// # Panics
    /// このメソッドは`i > 31`でパニックします
    #[must_use]
    fn add_context_flag(&self, i:usize) -> bool {
        self.ensure_add_context_index_within_range(i);
        self.as_ref()[1].get_bit(i)
    }

    /// `i`番目のAdd Context flagを1にする
    /// # Panics
    /// このメソッドは`i > 31`でパニックします
    #[must_use]
    fn set_add_context_flag(&mut self, i:usize)  {
        self.ensure_add_context_index_within_range(i);
        self.as_mut()[1].set_bit(i, true);
    }

    /// `i`番目のAdd Context flagを0にする
    /// # Panics
    /// このメソッドは`i > 31`でパニックします
    #[must_use]
    fn clear_add_context_flag(&mut self, i:usize)  {
        self.ensure_add_context_index_within_range(i);
        self.as_mut()[1].set_bit(i, false);
    }

    rw_field_cx!([7](0..=7), configuration_value, "Configuration Value", u8);
    rw_field_cx!([7](8..=15), interface_number, "Interface Number", u8);
    rw_field_cx!([7](8..=15), alternate_setting, "Alternate Setting", u8);

    #[doc(hidden)]
    fn ensure_drop_context_index_within_range(&self, i:usize) {
        assert!(
            (2..=31).contains(&i),
            "The index of Drop Context flag must be within 2..=31"
        );
    }
    #[doc(hidden)]
    fn ensure_add_context_index_within_range(&self, i:usize) {
        assert!(
            i <= 31,
            "The index of Add Context flag must be less than 32"
        );
    }
}

/// Device Context
/// Refer to [`DeviceHandler`] for the available methods
#[repr(C)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Device<const N:usize> {

}