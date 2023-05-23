#[macro_use]
macro_rules! write_csr {
    ($csr_number:literal) => {
        /// Writes the CSR
        #[inline]
        pub fn write(bits: usize) {
           unsafe {core::arch::asm!(concat!("csrrw x0, ", stringify!($csr_number), ", {0}"), in(reg) bits)}
        }
    };
}

#[macro_use]
macro_rules! read_csr {
    ($csr_number:literal) => {
        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            let bits: usize;
            unsafe {
                core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) bits)
            }
            bits
        }
    };
}

pub(crate) use read_csr;
pub(crate) use write_csr;
