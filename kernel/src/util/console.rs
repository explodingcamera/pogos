use core::{future::Future, pin::Pin, task::Poll};

#[macro_export]
macro_rules! print {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::util::print_args(format_args!($fmt $(,$($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {{
        $crate::print!($fmt $(,$($arg)+)?);
        $crate::util::print("\n");
    }};
    () => {
        $crate::util::print("\n");
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
        sbi::legacy::console_putchar(c) // TODO: replace with the new SBI debug extension once it's available in all SBI implementations
    });
}

struct DebugConsole();
impl Future for DebugConsole {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        match sbi::legacy::console_getchar() {
            Some(c) => Poll::Ready(c),
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

pub async fn get_char() -> u8 {
    DebugConsole().await
}
