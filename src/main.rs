slint::include_modules!();
use crossbeam::channel;
use device_query::{DeviceQuery, DeviceState};
use rdev::{listen, Event, EventType};
use slint::{ComponentHandle, LogicalPosition, SharedString};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

mod config;
use config::strucs::Setting;

fn main() -> Result<(), slint::PlatformError> {
    let (tx, rx) = channel::bounded::<i32>(1);

    let main_window = MainWindow::new()?;
    let conf = Arc::new(Mutex::new(Setting::default()));

    // main_window.show();
    let app_handle = main_window.as_weak();

    main_window.on_move_windows(move || {
        if let Some(main_window) = app_handle.upgrade() {
            let device_state = DeviceState::new();
            let mouse = device_state.get_mouse();
            let (x, y) = mouse.coords;
            let x = x - 15;
            let y = y - 15;

            main_window
                .window()
                .set_position(LogicalPosition::new(x as f32, y as f32));
        }
    });

    let conf_set_repeat = conf.clone();
    main_window.on_set_repeat(move |v| {
        if let Ok(mut conf) = conf_set_repeat.lock() {
            conf.set_repeat(v);
        }
    });

    main_window.on_record(move || {
        let _ = tx.send(1);
    });

   
    let conf_set_repeat_each = conf.clone();
    let main_window_set_repeat_each = main_window.clone_strong();
    main_window.on_set_repeat_each(move |v| {
        if v.is_empty() {
            if let Ok(mut conf) = conf_set_repeat_each.lock() {
                conf.set_repeat_each(1);
            }
            return;
        }

        let mut n = parse_int(v.as_str());

        if n.unwrap() > 10 {
            n = Some(10);
        }

        if let Ok(mut conf) = conf_set_repeat_each.lock() {
            conf.set_repeat_each(n.unwrap());
        }

        let each_return = SharedString::from(n.unwrap().to_string());

        main_window_set_repeat_each.set_repeat_each(each_return);
    });

    thread::spawn(|| {
        for _ in rx {
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
    let millis = start
        .duration_since(UNIX_EPOCH)
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

fn parse_int(input: &str) -> Option<i32> {
    input.trim().parse::<i32>().ok()
}
