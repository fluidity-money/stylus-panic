#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloy_primitives::Address;

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
pub mod host {
    #[link(wasm_import_module = "console")]
    unsafe extern "C" {
        pub fn log_txt(ptr: *const u8, len: usize);
    }

    #[link(wasm_import_module = "stylus_interpreter")]
    unsafe extern "C" {
        pub fn die(ptr: *const u8, len: usize, rc: i32);
    }

    #[link(wasm_import_module = "stylus_test_runner")]
    unsafe extern "C" {
        pub fn set_msg_sender(ptr: *const u8);
        pub fn wasm_request_rand(ptr: *const u8, len: i32);
    }
}

#[macro_export]
#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
macro_rules! harness_dbg {
    ($val:expr) => {
    {
        let tmp = $val;
        let msg = alloc::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!($val), &tmp);
        unsafe { $crate::log_txt(msg.as_ptr(), msg.len()) };
        tmp
    }};
    ($($vals:expr),+ $(,)?) => {
    {
        let tup = ($($vals),+);
        let msg = alloc::format!("[{}:{}] {} = {:#?}\n", file!(), line!(), stringify!(($($vals),+)), &tup);
        unsafe { $crate::log_txt(msg.as_ptr(), msg.len()) };
        tup
    }};
}

#[macro_export]
#[cfg(not(all(target_arch = "wasm32", feature = "stylus-interpreter")))]
macro_rules! harness_dbg {
    ($val:expr) => {};
    ($($vals:expr),+ $(,)?) => {};
}

#[cfg(all(not(feature = "std"), target_arch = "wasm32"))]
#[panic_handler]
fn panic(_msg: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "stylus-interpreter")]
    {
        let msg = alloc::format!("{_msg}");
        unsafe { host::die(msg.as_ptr(), msg.len(), 1) }
    }
    core::arch::wasm32::unreachable()
}

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
pub fn set_msg_sender(sender: Address) {
    unsafe { host::set_msg_sender(sender.0.as_ptr()) }
}

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
pub fn request_randomness<const SIZE: usize>() -> [u8; SIZE] {
    let mut b = [0u8; SIZE];
    unsafe { host::wasm_request_rand(b.as_mut_ptr(), SIZE as i32) }
    b
}
