
use rdev::{ Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config;
use config::data::{CONFIG_INSTANCE, CON_INSTANCE};



pub fn processEvent(event: Event) {
    let start = SystemTime::now();
    let millis = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let config = CONFIG_INSTANCE.get().unwrap();
    let con = CON_INSTANCE.get().unwrap();
    match event.event_type {
        EventType::KeyPress(rdev::Key::ControlLeft) => {
            con.set_ctr_press(true);
        }
        EventType::KeyRelease(rdev::Key::ControlLeft) => {
            con.set_ctr_press(false);
           
        }
        EventType::KeyPress(rdev::Key::KeyQ) => {
            if con.get_ctr_press() {
                if cfg!(debug_assertions) {
                println!("Key Q pressed");
                }
                config.set_recoding(false);
                con.tx.send(1).unwrap()
            }
          ;
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
}