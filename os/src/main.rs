#![no_main]
#![no_std]
mod lang_items;
// fn main() {
// //    println!("Hello, world!");
// }
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));