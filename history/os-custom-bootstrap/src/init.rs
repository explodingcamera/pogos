#[macro_export]
macro_rules! entry {
    ($path:path) => {
        global_asm!(include_str!("entry.asm"));

        #[export_name = "__main"]
        pub unsafe fn __main() -> ! {
            // type check the given path

            crate::init::__clear_bss();
            let f: fn() -> ! = $path;
            f()
        }
    };
}

pub fn __init() {}

pub fn __clear_bss() {
    extern "C" {
        static mut sbss: u8;
        static mut ebss: u8;
    }

    unsafe {
        let start = &mut sbss as *mut u8 as usize;
        let end = &mut ebss as *mut u8 as usize;
        for i in start..end {
            (i as *mut u8).write_volatile(0);
        }
    }
}
