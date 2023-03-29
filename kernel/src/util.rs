use numtoa::NumToA;

#[inline]
pub fn print(t: &str) {
    for c in t.chars() {
        let c: u8 = c.try_into().unwrap_or(0);
        sbi::legacy::console_putchar(c)
    }
}

pub fn print_usize(u: usize) {
    let mut buf = [0u8; 20];
    print(u.numtoa_str(10, &mut buf));
}
