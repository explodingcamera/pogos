#[inline]
pub fn print(t: &str) {
    for c in t.chars() {
        let c: u8 = c.try_into().unwrap_or(0);
        sbi::legacy::console_putchar(c)
    }
}
