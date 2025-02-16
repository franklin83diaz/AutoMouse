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
pub struct MouseEvent {
    action: u8,
    button: u8,
    x: i32,
    y: i32,
    time: i32,
}

impl MouseEvent {
    pub fn new(action: u8, button: u8, x: i32, y: i32, time: i32) -> Self {
        MouseEvent {
            action,
            button,
            x,
            y,
            time,
        }
    }

    pub fn get_tuple(&self) -> (u8, u8, i32, i32, i32) {
        (self.action, self.button, self.x, self.y, self.time)
    }
}

#[derive(Default)]
// mouse_event_list
pub struct MouseEventList {
    name: Mutex<String>,
    miliseconds_runing: Mutex<i32>,
    pub mouse_events: Mutex<Vec<MouseEvent>>,
}

impl MouseEventList {
    pub fn set_name(&self, name: String) {
        let mut data = self.name.lock().unwrap();
        *data = name;
    }

    pub fn get_name(&self) -> String {
        let data = self.name.lock().unwrap();
        data.clone()
    }

    pub fn set_miliseconds_runing(&self, seconds_runing: i32) {
        let mut data = self.miliseconds_runing.lock().unwrap();
        *data = seconds_runing;
    }

    pub fn get_miliseconds_runing(&self) -> i32 {
        let data = self.miliseconds_runing.lock().unwrap();
        *data
    }

    pub fn add_mouse_event(&self, mouse_event: MouseEvent) {
        let mut data = self.mouse_events.lock().unwrap();
        data.push(mouse_event);
    }
}

pub static MOUSE_EVENT_LIST: OnceLock<MouseEventList> = OnceLock::new();
