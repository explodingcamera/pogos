pub fn shutdown() -> ! {
    let _ = sbi::system_reset::system_reset(
        sbi::system_reset::ResetType::Shutdown,
        sbi::system_reset::ResetReason::NoReason,
    );
    unreachable!("System reset failed");
}
