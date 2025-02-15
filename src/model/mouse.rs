use std::sync::Mutex;
use std::sync::OnceLock;


// -Action- 
// 0: move
// 1: button press
// 2: button release
// 3: scroll
//
// -button-
// 0: reserve
// 1: left
// 2: right
// 3: middle
struct mouse_event {
    action: u8,
    button: u8,
    x: i32,
    y: i32,
    time: i32,
}

#[derive(Default)]
// mouse_event_list
pub struct mouse_event_list {
    name: Mutex<String>,
    seconds_runing: Mutex<i32>,
    pub   mouse_events: Mutex<Vec<mouse_event>>,
}

impl mouse_event_list {
    pub fn set_name(&self, name: String) {
        let mut data = self.name.lock().unwrap();
        *data = name;
    }

    pub fn get_name(&self) -> String {
        let data = self.name.lock().unwrap();
        data.clone()
    }

    pub fn set_seconds_runing(&self, seconds_runing: i32) {
        let mut data = self.seconds_runing.lock().unwrap();
        *data = seconds_runing;
    }

    pub fn get_seconds_runing(&self) -> i32 {
        let data = self.seconds_runing.lock().unwrap();
        *data
    }

    pub fn add_mouse_event(&self, mouse_event: mouse_event) {
        let mut data = self.mouse_events.lock().unwrap();
        data.push(mouse_event);
    }

 
    
}

pub static MOUSE_EVENT_LIST: OnceLock<mouse_event_list> = OnceLock::new();