#![no_std]

use core::panic::PanicInfo;

use pokeemerald::{
    gTasks, AddTextPrinterParameterized, GetPlayerTextSpeedDelay, MgbaPrintf, RunTextPrinters,
    Task, FONT_NORMAL,
};

mod charmap;
mod slice_write;

use slice_write::Write;

static HELLO: &[u8] = &pokestr!(b"Hello from rust!");

unsafe fn g_tasks(task_id: u8) -> *mut Task {
    #[allow(static_mut_refs)]
    gTasks.as_mut_ptr().add(task_id as usize)
}

#[no_mangle]
pub extern "C" fn Task_HandleRust(task_id: u8) {
    unsafe {
        let mut text: [u8; 100] = [0; 100];
        let task = *g_tasks(task_id);
        _ = write!(
            &mut text.as_mut_slice(),
            "task_id:{task_id} {}\0",
            task.data[15]
        );
        MgbaPrintf(2, text.as_ptr());
        if task.data[15] > 0 {
            RunTextPrinters();
            return;
        }
        (*g_tasks(task_id)).data[15] = 1;
        AddTextPrinterParameterized(
            0,
            FONT_NORMAL as _,
            HELLO.as_ptr(),
            0,
            0,
            GetPlayerTextSpeedDelay(),
            None,
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        let bytes = _info.message().as_str().unwrap_or_default().as_bytes();
        let mut text = [0; 100];
        text.copy_from_slice(bytes);
        text[99] = 0;
        MgbaPrintf(0, text.as_ptr());
    }
    loop {}
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod pokeemerald {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
