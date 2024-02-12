use alloc::{ffi::CString, vec, vec::Vec};

pub fn debug_write(t: &[u8]) {
    let num_bytes = t.len();

    unsafe {
        // get the address of the string
        let base_addr = t.as_ptr() as usize;
        let base_addr_lo = base_addr & 0xFFFFFFFF;
        let base_addr_hi = (base_addr >> 32);

        let error: isize;
        let value: usize;

        core::arch::asm!(
            "ecall",
            inlateout("a0") num_bytes => error,
            inlateout("a1") base_addr_lo => value,
            in("a2") base_addr_hi,
            in("a6") 0x0,
            in("a7") 0x4442434E,
        );
    }
}

pub fn debug_read(num_bytes: usize) -> Option<Vec<u8>> {
    let mut buffer = vec![0; num_bytes];

    unsafe {
        // get the address of the string
        let base_addr = buffer.as_mut_ptr() as usize;
        let base_addr_lo = base_addr & 0xFFFFFFFF;
        let base_addr_hi = (base_addr >> 32);

        let error: isize;
        let value: usize;

        core::arch::asm!(
            "ecall",
            inlateout("a0") num_bytes => error,
            inlateout("a1") base_addr_lo => value,
            in("a2") base_addr_hi,
            in("a6") 0x1,
            in("a7") 0x4442434E,
        );
    }

    Some(buffer)
}
