use std::fmt::Debug;

pub fn print_debug(desc: &str, val: impl Debug) {
    #[cfg(debug_assertions)]
    println!("--------------------------------------------");
    #[cfg(debug_assertions)]
    println!("{desc}:");
    #[cfg(debug_assertions)]
    println!("{:#?}", val);
}
