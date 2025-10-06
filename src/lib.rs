#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
#[link(wasm_import_module = "console")]
extern "C" {
    pub fn log_txt(ptr: *const u8, len: usize);
}

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
#[link(wasm_import_module = "stylus_interpreter")]
#[allow(unused_code)]
extern "C" {
    fn die(ptr: *const u8, len: usize, rc: i32);
}

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
#[link(wasm_import_module = "stylus_test_runner")]
#[allow(unused_code)]
extern "C" {
    fn set_msg_sender(ptr: i32);
    fn wasm_request_rand(ptr: i32, len: i32);
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
        unsafe { die(msg.as_ptr() as i32, msg.len() as i32, 1) }
    }
    core::arch::wasm32::unreachable()
}

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
pub fn set_msg_sender(sender: Address) {
    set_msg_sender(sender.0.as_ptr())
}

#[cfg(all(target_arch = "wasm32", feature = "stylus-interpreter"))]
pub fn request_randomness<SIZE: usize>() -> [u8; SIZE] {
    let mut b = [0u8; SIZE];
    wasm_request_rand(b.as_mut_ptr(), SIZE);
    b
}
