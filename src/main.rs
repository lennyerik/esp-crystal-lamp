#![no_std]
#![no_main]

use esp_alloc as _;
use esp_backtrace as _;
extern crate alloc;

use esp_println::println;

#[esp_hal::main]
fn main() -> ! {
    println!("Hello world from chip!");

    panic!("DONE");
}
