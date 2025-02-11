use rdev::listen;
use std::thread::{self, sleep};

use crate::mouse::process;


pub fn run_all_threads() {
      // Listen  rdev for get mouse and keyboard events
      thread::spawn(|| {
        if let Err(error) = listen(|e|{
            process::event(e);
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

    
}