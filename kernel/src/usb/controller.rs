use core::num::NonZeroUsize;
use xhci::accessor::Mapper;

pub struct Controller {
}

pub struct MemoryMapper {

}
impl Mapper for MemoryMapper {
    unsafe fn map(&mut self, phys_start: usize, bytes: usize) -> NonZeroUsize {
        todo!()
    }

    fn unmap(&mut self, virt_start: usize, bytes: usize) {
        todo!()
    }
}