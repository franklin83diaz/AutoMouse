use rdev::{simulate, EventType, SimulateError};
use std::{thread, time};

pub fn send(event_type: &EventType, milisec: i32) {
    let milisec_delay = time::Duration::from_millis(8);
    thread::sleep(milisec_delay);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    //let delay = time::Duration::from_millis(20);
    // Let ths OS catchup (at least MacOS)
   // thread::sleep(delay);
}