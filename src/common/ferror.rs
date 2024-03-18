#[macro_export]
/// Print a lil cool error thingy idk
macro_rules! ferror {
    ($($rest:tt)*) => {
        let string = std::fmt::format(std::format_args!($($rest)*));
        std::eprintln!("\x1b[1;31mE\x1b[0m: {string}");
    };
}
