use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::page_table::FrameError;


pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}


/// 有効なレベル4テーブルへの可変参照を返す。
///
/// この関数はunsafeである：全物理メモリが、渡された上で
/// `physical_memory_offset`(だけずらしたうえ)で
/// 仮想メモリへとマップされていることを呼び出し元が保証しなければならない
/// また、`&mut`の参照が複数の名称を持つことは`mutable aliasing`といい、
/// 未定義動作につながるため、この関数は一度しか呼び出してはいけない。
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

#[warn(dead_code)]
pub unsafe fn translate_addr(addr:VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

#[warn(dead_code)]
fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    let (level_4_table_frame, _) = Cr3::read();
    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => { frame }
            Err(FrameError::FrameNotPresent) => { return None },
            Err(FrameError::HugeFrame) => panic!("huge pages not supported")
        };
    }

    Some(frame.start_address() + u64::from(addr.page_offset()))
}