#![no_std]

use core::panic::PanicInfo;

use pokeemerald::MgbaPrintf;

#[no_mangle]
pub extern "C" fn rust_toast() {
    panic!("Hello World!")
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        MgbaPrintf(0, _info.message().as_str().unwrap_or_default().as_ptr());
    }
    loop {}
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod pokeemerald {
    include!("bindings.rs");
}
