slint::include_modules!();
use crossbeam::channel;
use device_query::{DeviceQuery, DeviceState};
use rdev::{listen, Event};
use slint::{invoke_from_event_loop, ComponentHandle, LogicalPosition, SharedString};
use std::thread::{self, sleep};

use std::time::Duration;
mod config;
use config::set::repeat_each;
use config::data::{Communication, Setting, CONFIG_INSTANCE, CON_INSTANCE};

mod mouse;
use mouse::process;

fn main() -> Result<(), slint::PlatformError> {
    //  let (tx, rx) = channel::bounded::<i32>(1);
    let con = CON_INSTANCE.get_or_init(Communication::new);

    let main_window = MainWindow::new()?;

    let handle_weak = main_window.as_weak();

    thread::spawn(move || loop {
        let mut t = 0;
        loop {
            let config = CONFIG_INSTANCE.get().unwrap();
            if !config.get_recoding() {
                break;
            }

            let handle_copy = handle_weak.clone();

            let t_string = format!("{:02}:{:02}:{:02}", t / 3600, (t % 3600) / 60, t % 60);
            let _ = invoke_from_event_loop(move || {
                handle_copy.unwrap().set_time(SharedString::from(t_string))
            });
            sleep(Duration::from_millis(1000));
            t += 1;
        }
    });

    let handle_weak2 = main_window.as_weak();
    thread::spawn(move || loop {
        let data = con.rx.recv().unwrap();
        println!("Received: {}", data);
        let handle_copy2 = handle_weak2.clone();
        let _ = invoke_from_event_loop(move || handle_copy2.unwrap().set_recording(false));
    });

    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);

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

    main_window.on_set_repeat(move |v| {
        conf.set_repeat(v);
    });

    main_window.on_record(move || {
        conf.set_recoding(true);
    });

    let main_window_set_repeat_each = main_window.clone_strong();
    main_window.on_set_repeat_each(move |v| {
        repeat_each(v, &main_window_set_repeat_each);
    });

    thread::spawn(|| {
        if let Err(error) = listen(callback) {
            eprintln!("Error: {:?}", error);
        }
    });

    main_window.run()
}

fn callback(event: Event) {
    process::processEvent(event);
}
