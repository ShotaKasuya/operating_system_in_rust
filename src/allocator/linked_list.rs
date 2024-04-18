

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
        todo!()
    }

    /// 与えられたメモリ領域をリストの先頭に追加する。
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        todo!()
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