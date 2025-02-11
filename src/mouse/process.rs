
use rdev::{ Event, EventType};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config;
use config::strucs::{CONFIG_INSTANCE, CON_INSTANCE};


pub fn processEvent(event: Event) {
    let start = SystemTime::now();
    let millis = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
    let config = CONFIG_INSTANCE.get().unwrap();
    match event.event_type {
        EventType::KeyPress(rdev::Key::ControlLeft) => {
            println!("Left Control");
            config.set_recoding(false);
            let con = CON_INSTANCE.get().unwrap();
            con.tx.send(1).unwrap();
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