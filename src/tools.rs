use std::fmt::Debug;

#[cfg(debug_assertions)]
pub fn print_debug(desc: &str, val: impl Debug) {
    println!("--------------------------------------------");
    println!("{desc}:");
    println!("{:#?}", val);
}
