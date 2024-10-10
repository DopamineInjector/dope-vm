use std::sync::OnceLock;

pub struct Mmu {
    start: i32,
    current_offset: i32,
}

impl Mmu {
    pub fn new(start: i32) -> Self {
        return Mmu { start, current_offset: 0 }
    }

    pub fn allocate_memory(&mut self, size: i32) -> i32 {
        let ptr = self.current_offset + self.start;
        self.current_offset += size + 8;
        ptr
    }
}

pub static mut MMU: OnceLock<Mmu> = OnceLock::new();

