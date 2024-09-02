use core::num::NonZeroUsize;
use accessor::Mapper;

/// Temporary implementation
pub struct MemoryMapper {

}

#[allow(unused_variables)]
impl Mapper for MemoryMapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        NonZeroUsize::new_unchecked(phys_start)
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {

    }
}

impl MemoryMapper {
    pub fn new() -> Self {
        Self {}
    }
}