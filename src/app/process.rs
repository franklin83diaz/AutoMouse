use rdev::{Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{self, data, data::map_key};
use chrono::Local;
use config::data::{CONFIG_INSTANCE, CON_INSTANCE, MOUSE_TRACKER_LIST};
use crate::model::sql;

pub fn event(event: Event) {
    let start = SystemTime::now();
    let millis = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let config = CONFIG_INSTANCE.get().unwrap();
    let con = CON_INSTANCE.get().unwrap();
    let mtl = MOUSE_TRACKER_LIST.get_or_init(data::MouseTrackerList::default);
    let mk = map_key(config.get_key_stop());
    let _key_stop = mk.1;
    match event.event_type {
        EventType::KeyPress(rdev::Key::ControlLeft) => {
            con.set_ctr_press(true);
        }
        EventType::KeyRelease(rdev::Key::ControlLeft) => {
            con.set_ctr_press(false);
        }
        EventType::KeyPress(key) if key == _key_stop => {
            if con.get_ctr_press() {
                if cfg!(debug_assertions) {
                    println!("Key {} pressed", mk.0);
                }

                let now = Local::now();
                mtl.set_end_time_unix(now.timestamp() as i32);
                println!("------------------------------");
                println!("{}", mtl);
                println!("------------------------------");
                config.set_recoding(false);
                con.tx.send(1).unwrap();
                sql::set_mouse_macro();
            }
            return;
        }

        EventType::MouseMove { x, y } => {
            if config.get_recoding() {
                println!("Mouse moved to: x = {},y = {} , time = {}", x, y, millis);
            }
        }
        EventType::ButtonPress(button) => {
            println!("Mouse button pressed: {:?}", button);
        }
        EventType::ButtonRelease(button) => {
            println!("Mouse button released: {:?}", button);
        }
        _ => {}
    }

    if config.get_recoding() {
        let mouse_tracker = data::MouseTracker::new(1, false, false, 0, 0);
        mtl.add(mouse_tracker);
    }
}
