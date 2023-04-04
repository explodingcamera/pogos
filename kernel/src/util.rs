#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::util::print_args(format_args!(concat!($fmt, "\n") $(,$($arg)+)?));
    }
}

struct Writer {}

pub fn print_args(t: core::fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = Writer {};
    writer.write_fmt(t).unwrap();
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

pub fn print(t: &str) {
    t.chars().for_each(|c| {
        let c: u8 = c.try_into().unwrap_or(b'?');
        sbi::legacy::console_putchar(c)
    });
}
