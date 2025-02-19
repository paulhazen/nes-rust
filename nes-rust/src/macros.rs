#[macro_export]
macro_rules! hex {
    ($val:literal) => {
        u8::from_str_radix(concat!("0x", stringify!($val)), 16).unwrap()
    };
}