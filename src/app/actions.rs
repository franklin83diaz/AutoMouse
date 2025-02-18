use crate::app;
use crate::crud;
use rdev::{simulate, EventType, SimulateError};
use std::{thread, time};

fn send(event_type: &EventType, milliseconds: i32) {
    let milliseconds_delay = time::Duration::from_millis(milliseconds as u64);
    thread::sleep(milliseconds_delay);

    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }

    // let delay = time::Duration::from_millis(20);
    // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
}

pub fn replay(id: i32) {
    let v1 = crud::sql::get_mouse_macro_list(id);
    let mut temp_time = 0;

    for me in v1 {
        let (actions, button, time, x_point, y_point) = me.get_tuple();
        let time_sleep = time - temp_time;
        temp_time = time;
     

        if actions.clone() as i32 == 1 {
            app::actions::send(
                &EventType::MouseMove {
                    x: x_point as f64,
                    y: y_point as f64,
                },
                time_sleep,
            );
        }

        if actions.clone() as i32 == 2 {
            match button.clone() as i32 {
                1 => app::actions::send(&EventType::ButtonPress(rdev::Button::Left), time_sleep),
                2 => app::actions::send(&EventType::ButtonPress(rdev::Button::Right), time_sleep),
                _ => app::actions::send(&EventType::ButtonPress(rdev::Button::Middle), time_sleep),
            }
        }

        if actions.clone() as i32 == 3 {
            match button.clone() as i32 {
                1 => app::actions::send(&EventType::ButtonRelease(rdev::Button::Left), time_sleep),
                2 => app::actions::send(&EventType::ButtonRelease(rdev::Button::Right), time_sleep),
                _ => {
                    app::actions::send(&EventType::ButtonRelease(rdev::Button::Middle), time_sleep)
                }
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("Replay finished");
    }
}
