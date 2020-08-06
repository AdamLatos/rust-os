#![no_std]
#![no_main]

// set up testing framework

#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// set up vga buffer

use core::panic::PanicInfo;
use os::println;

// main

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", 123);
    
    #[cfg(test)]
    test_main();
    
    loop{}
}

// tests

#[test_case]
fn test_trivial_assertion() {
    assert_eq!(1, 1);
}

// set up panic handler

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}