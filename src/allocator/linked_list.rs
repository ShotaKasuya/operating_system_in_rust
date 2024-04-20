use core::mem;

use crate::allocator::align_up;



struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    // 空のLinkedListAllocatorを作る
    pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

    /// 与えられたヒープ領域でアロケータを初期化する。
    /// この関数は'unsafe'である。なぜなら、呼び出し元は渡すヒープ境界が
    /// 有効でヒープが未使用であることを保証しなければならないからである。
    /// このメソッドは一度しかよばれてはならない。
    pub unsafe fn init(&mut self, heap_start:usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    /// 与えられたメモリ領域をリストの先頭に追加する。
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // 解放された領域が`ListNode`を格納出来ることを確かめる
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);
        assert!(size >= mem::size_of::<ListNode>());

        // 新しいリストノードを作り、それをリストの先頭に追加する
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }

    /// 与えられたサイズの解放された領域を探し、リストからそれを取り除く。
    /// リストノードと割り当ての開始アドレスからなるタプルを返す。
    fn find_region(&mut self, size: usize, align: usize)
        -> Option<(&'static mut ListNode, usize)> {
            // 現在のリストノードへの参照。繰り返すごとに更新していく
            let mut current = &mut self.head;
            // 連結リストから十分大きな領域を探す
            while let Some(ref mut region) = current.next {
                if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                    // 領域が割り当てに適している -> リストから除く
                    let next = region.next.take();
                    let ret = Some((current.next.take().unwrap(), alloc_start));
                    current.next = next;
                    return ret;
                } else {
                    // 割り当てに適していない -> 次の領域へ
                    current = current.next.as_mut().unwrap();
                }
            }

            None
        }
}

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>
}

impl ListNode {
    const fn new(size: usize) -> Self{
        ListNode{
            size: size,
            next: None,
        }
    }
    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}