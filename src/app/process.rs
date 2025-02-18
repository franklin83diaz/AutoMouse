use rdev::{Event, EventType};
use chrono::Local;
use config::data::{CONFIG_INSTANCE, CON_INSTANCE};

use crate::config::{self,  data::map_key};
use crate::state::global::RECODIND_META_DATA;
use crate::model::mouse::{MOUSE_EVENT_LIST, MouseEvent, MouseAction, MouseButton};
use crate::crud::sql;

pub fn event(event: Event) {
 
    let config = CONFIG_INSTANCE.get().unwrap();
    let con = CON_INSTANCE.get().unwrap();
    let rmd = RECODIND_META_DATA.get().unwrap();
    let mel = MOUSE_EVENT_LIST.get().unwrap();
    let mk = map_key(config.get_key_stop());
    let _key_stop = mk.1;

    let mut action: MouseAction = MouseAction::Unknown;
    let mut button: MouseButton = MouseButton::Unknown;
    let mut xpoint: i32 = 0;
    let mut ypoint: i32  = 0;
 

    match event.event_type {

        //Control Key
        EventType::KeyPress(rdev::Key::ControlLeft) => {
            rmd.set_ctr_press(true);
        }
        EventType::KeyRelease(rdev::Key::ControlLeft) => {
            rmd.set_ctr_press(false);
        }
        // Stop recoding
        EventType::KeyPress(key) if key == _key_stop => {
            if rmd.get_ctr_press() {
                if cfg!(debug_assertions) {
                    println!("Key {} pressed", mk.0);
                }

                let now: chrono::DateTime<Local> = Local::now();
                let milliseconds_runing = now.timestamp_millis() as i32 - rmd.get_start_time_unix();
                mel.set_milliseconds_runing(milliseconds_runing);

                config.set_recoding(false);

                if cfg!(debug_assertions) {
                    println!("Seconds running: {}", milliseconds_runing);
                }

                // Send sinal to thread for set_recording(false)
                con.tx.send(1).unwrap(); 
                sql::save_mouse_macro();
            }
            return;
        }

        // Mouse events
        EventType::MouseMove { x, y } => {
            if config.get_recoding() {
                xpoint = x as i32;
                ypoint = y as i32;
                action = MouseAction::Move;
            }
        }
        
        EventType::ButtonPress(b) => {
            button = match b {
                rdev::Button::Left => MouseButton::Left,
                rdev::Button::Right => MouseButton::Right,
                _ => MouseButton::Middle,
            };
            action = MouseAction::Press;
        }

        EventType::ButtonRelease(b) => {
            button = match b {
                rdev::Button::Left => MouseButton::Left,
                rdev::Button::Right => MouseButton::Right,
                _ => MouseButton::Middle,
            };
            action = MouseAction::Release;
        }

        _ => {}
    }
  

    if config.get_recoding() {
        let now: chrono::DateTime<Local> = Local::now();
        let milliseconds_runing = now.timestamp_millis() as i32 - rmd.get_start_time_unix();
        let mouse_event = MouseEvent::new(action.clone(), button, milliseconds_runing, xpoint,ypoint );
        if action as i32 != 0{
            mel.add_mouse_event(mouse_event);
        }
    }
}
