#[cfg(all(feature = "a", feature = "b"))]
compile_error!("cannot impl both a and b");

pub const VALUE: u8 = if cfg!(feature = "a") { 2 } else if cfg!(feature = "b") { 1 } else { 0 };