use core::fmt::Arguments;

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::gfx::print::print(format_args!($($arg)*)));
}

pub fn print(_args: Arguments<'_>) {
    todo!()
}
