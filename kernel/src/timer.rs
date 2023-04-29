use crate::util::Result;

pub unsafe fn init_timer_interrupt() {
    use riscv::register::sie;

    // enable supervisor timer interrupt
    sie::set_stimer();
}

// get the current time in milliseconds
pub fn get_time() -> Result<u64> {
    let timebase_freq = crate::dtb::DEVICE_TREE
        .get()
        .expect("device tree not initialized")
        .timebase_freq()?;

    Ok(riscv::register::time::read64() / (timebase_freq as u64 / 1000))
}

pub fn set_interrupt(time: u64) -> Result<()> {
    sbi::timer::set_timer(time).map_err(|_| "set timer failed")
}
