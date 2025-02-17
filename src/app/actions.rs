use rdev::{simulate, EventType, SimulateError};
use std::{thread, time};
use crate::crud;
use crate::app;

fn send(event_type: &EventType, milisec: i32) {
    println!("{}", milisec);
    let milisec_delay = time::Duration::from_millis(milisec as u64);
    thread::sleep(milisec_delay);
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

pub fn replay(id : i32) {
    let v1 =crud::sql::get_mouse_macro_list(id);
    let mut temp_time =0;

    for me in v1 {
    let (  actions, button,time, xpoint, ypoint) = me.get_tuple();
        
       let  time_sleep = time - temp_time;
       temp_time = time;
    if actions.clone() as i32 == 1 {
        app::actions::send(&EventType::MouseMove { x: xpoint as f64, y: ypoint as f64 }, time_sleep);
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
            _ => app::actions::send(&EventType::ButtonRelease(rdev::Button::Middle), time_sleep),
        }   
    }
}
}