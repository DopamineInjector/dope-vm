use std::ffi::CString;

use wasmtime::{Memory, StoreContext, StoreContextMut};

use super::mmu::{Mmu, MMU};

pub fn get_string_from_memory(ptr: usize, len: usize, memory: Memory, store: StoreContext<()>) -> String {
    let mut buf = vec![0; len].into_boxed_slice();
    let _ = memory.read(store, ptr, &mut buf);
    String::from_utf8(buf.to_vec()).unwrap()
}

pub fn put_string_in_memory(content: &str, memory: Memory, store: StoreContextMut<()>) -> i32{
    let mut ptr = 0;
    unsafe {
        let mmu = MMU.get_mut_or_init(|| Mmu::new(0x2710));
        let cstring = CString::new(content).unwrap();
        let buffer = cstring.as_bytes();
        let allocated_offset = mmu.allocate_memory(buffer.len() as i32);
        let _ = memory.write(store, allocated_offset as usize, buffer);
        ptr = allocated_offset;
    }
    ptr
}
