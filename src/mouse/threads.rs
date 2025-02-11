
use rdev::listen;
use std::thread::{self, sleep};
use std::time::Duration;

use crate::mouse::process;
use slint::{invoke_from_event_loop, ComponentHandle, SharedString};
use crate::config::data::{CONFIG_INSTANCE, Communication, CON_INSTANCE};



pub fn run_all_threads( main_window: &crate::slint_generatedMainWindow::MainWindow) {
    let handle_weak = main_window.as_weak();
      // Listen  rdev for get mouse and keyboard events
      thread::spawn(|| {
        if let Err(error) = listen(|e|{
            process::event(e);
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

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


    let con = CON_INSTANCE.get_or_init(Communication::new);
    let handle_weak2 = main_window.as_weak();
    thread::spawn(move || loop {
        let data = con.rx.recv().unwrap();
        println!("Received: {}", data);
        let handle_copy2 = handle_weak2.clone();
        let _ = invoke_from_event_loop(move || handle_copy2.unwrap().set_recording(false));
    });

    
}