#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]


mod samus;



#[skyline::main(name = "smashline_test")]
pub fn main() {
    samus::install();
}