use rdev::{Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{self, data, data::map_key};
use crate::model::sql;
use crate::state::global::RECODIND_META_DATA;
use crate::model::mouse::{MOUSE_EVENT_LIST, mouse_event_list};
use chrono::Local;
use config::data::{CONFIG_INSTANCE, CON_INSTANCE};

pub fn event(event: Event) {
    let start = SystemTime::now();
    let millis = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let config = CONFIG_INSTANCE.get().unwrap();
    let con = CON_INSTANCE.get().unwrap();
    let rmd = RECODIND_META_DATA.get().unwrap();
    let mel = MOUSE_EVENT_LIST.get().unwrap();
    let mk = map_key(config.get_key_stop());
    let _key_stop = mk.1;

    let mut xpoint: f64 = -1.0;
    let mut ypoint: f64 = -1.0;
    let mut left_click = false;
    let mut right_click = false;

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

                let now: chrono::DateTime<Local> = Local::now();
                let seconds_runing = now.timestamp() as i32 - rmd.get_start_time_unix();
                mel.set_seconds_runing(seconds_runing);

                println!("now: {},", now.timestamp() as i32);
                println!("start time: {} ", rmd.get_start_time_unix());


                config.set_recoding(false);

                if cfg!(debug_assertions) {
                    println!("Seconds running: {}", seconds_runing);
                }

                // TODO: Review this
                con.tx.send(1).unwrap(); //This
               // sql::save_mouse_macro();
            }
            return;
        }
        EventType::MouseMove { x, y } => {
            if config.get_recoding() {
                println!("Mouse moved to: x = {},y = {} , time = {}", x, y, millis);
                xpoint = x;
                ypoint = y;
            }
        }
        EventType::ButtonPress(button) => {
            left_click = button == rdev::Button::Left;
            right_click = button == rdev::Button::Right;
            println!("Mouse button pressed: {:?}", button);
        }

        EventType::ButtonRelease(button) => {
            println!("Mouse button released: {:?}", button);
        }
        _ => {}
    }
  

    if config.get_recoding() {
        // let mouse_tracker =
        //     data::MouseTracker::new(millis, left_click, right_click, xpoint, ypoint);
        // mtl.add(mouse_tracker);
     
    }
}
