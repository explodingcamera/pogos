use super::sbi;
use core::{future::Future, pin::Pin, task::Poll};

#[macro_export]
macro_rules! print {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::util::SBIConsole::print_args(core::format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {{
        $crate::print!($fmt $(,$($arg)+)?);
        $crate::util::SBIConsole::print("\n");
    }};
    () => {
        $crate::util::SBIConsole::print("\n");
    }
}

pub struct SBIConsole();
impl SBIConsole {
    pub fn read_byte() -> Option<u8> {
        sbi::debug_read(1).map(|v| v[0])
    }

    pub fn print(s: &str) {
        sbi::debug_write(s.as_bytes());
    }

    pub fn print_args(t: core::fmt::Arguments) {
        use core::fmt::Write;
        SBIConsole().write_fmt(t).unwrap();
    }
}

impl core::fmt::Write for SBIConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        SBIConsole::print(s);
        Ok(())
    }
}
