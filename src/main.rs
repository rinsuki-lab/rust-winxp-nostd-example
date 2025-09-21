#![no_std]
#![no_main]

use windows_sys::{core::w,Win32::System::Threading::ExitProcess,Win32::UI::WindowsAndMessaging::{MessageBoxW,MB_OK}};

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: core::ffi::c_int, _argv: *const *const core::ffi::c_char) -> isize {
    unsafe {
        MessageBoxW(
            0 as _, w!("Hello, world!"), w!("Hello from Rust"), MB_OK
        );
    }
    0
}

#[panic_handler]
unsafe fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        ExitProcess(0x1)
    }
}
