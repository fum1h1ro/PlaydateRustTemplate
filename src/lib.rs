#![feature(lang_items)]
#![no_std]

mod bindings;
use core::panic::PanicInfo;
use bindings::*;
use core::ffi::CStr;


#[no_mangle]
pub extern "C" fn rust_event_handler(
    playdate: *mut PlaydateAPI,
    event: PDSystemEvent,
    _arg: u32,
) -> ::core::ffi::c_int {
    if event == PDSystemEvent_kEventInit {
        //let sys = unsafe { (*playdate).system };
        //let c_string: &'static [u8] = b"Hello, Rust World! %d\0";
        ////let cstr = CStr::from_bytes_with_nul(b"hello\0").unwrap();
        ////let c_str: *const c_char = cstr.as_ptr().cast();
        //if let Some(log_to_console) = unsafe { (*sys).logToConsole } {
        //    //let c_string: [u8; 6] = [72, 101, 108, 108, 111, 0];
        //    //let c_string = CStr::from_bytes_with_nul(b"hello\0").unwrap();
        //    //let c_str: *const c_char = c_string.as_ptr().cast();


        //    unsafe {
        //        let s = CStr::from_bytes_with_nul(b"Hello, Rust World!\0").unwrap().to_bytes();
        //        log_to_console(s.as_ptr().cast());
        //    }
        //    //unsafe { log_to_console(b"Hello, Rust World! %d\0".as_ptr().cast(), 2024) };


        //    //log_to_console(c_string.as_ptr().cast());
        //}
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn rust_update(
    playdate: *mut PlaydateAPI
) -> ::core::ffi::c_int {
    unsafe {
        let sys = (*playdate).system;
        if let Some(draw_fps) = (*sys).drawFPS {
            draw_fps(100, 100);
        }
    }
    return 1;
}

#[no_mangle]
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
#[lang = "eh_personality"]
pub extern fn eh_personality() {}

