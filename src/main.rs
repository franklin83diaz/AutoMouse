slint::include_modules!();
use device_query::{DeviceQuery, DeviceState};
use slint::{ComponentHandle, LogicalPosition};
use rdev::{listen, Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread};
use crossbeam::channel;


fn main() -> Result<(), slint::PlatformError> {
    let (tx, rx) = channel::bounded::<i32>(1);

    let main_window = MainWindow::new()?;
  
    // main_window.show();
    let app_handle = main_window.as_weak();
    main_window.on_move_windows(move || {
        if let Some(main_window) = app_handle.upgrade() {
            let device_state = DeviceState::new();
            let mouse = device_state.get_mouse();
            let (x, y) = mouse.coords;
            let x =x -15;
            let y = y-15;

            main_window
                .window()
                .set_position(LogicalPosition::new(x as f32, y as f32));
        }

    });

    main_window.on_record(move || {
        let _ =  tx.send(1);
    });

    thread::spawn(|| {
        for _ in rx  {
            if let Err(error) = listen(callback) {
                eprintln!("Error: {:?}", error);
            }
        }
            
    });


    main_window.run()
}

fn callback(event: Event) {
    println!("{:?}", event);
    
    let start = SystemTime::now();
    let millis = start.duration_since(UNIX_EPOCH)
        .expect("El tiempo debe ser posterior a UNIX_EPOCH")
        .as_millis();
    
  
    match event.event_type {

        EventType::KeyPress(rdev::Key::ControlLeft) => {
            println!("Left Control");
        }
        EventType::MouseMove { x, y } => {
            println!("Mouse moved to: x = {},y = {} , time = {}", x, y, millis);
        }
        EventType::ButtonPress(button) => {
            println!("Mouse button pressed: {:?}", button);
        }
        EventType::ButtonRelease(button) => {
            println!("Mouse button released: {:?}", button);
        }
        _ => {}
    }
}