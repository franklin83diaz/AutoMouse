use std::sync::Mutex;
use std::sync::OnceLock;

#[derive(Clone)]
pub enum MouseAction {
    Unknown,
    Move,
    Press,
    Release,
    Scroll,
}

#[derive(Clone)]
pub enum MouseButton {
    Unknown,
    Left,
    Right,
    Middle,
}

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
    action: MouseAction,
    button: MouseButton,
    x: i32,
    y: i32,
    time: i32,
}

impl MouseEvent {
    pub fn new(action:MouseAction , button:MouseButton , time: i32, x: i32, y: i32) -> Self {
        MouseEvent {
            action,
            button,
            time,
            x,
            y,
        }
    }

    pub fn get_tuple(&self) -> (MouseAction, MouseButton, i32, i32, i32) {
        (self.action.clone(), self.button.clone(), self.time, self.x, self.y)
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
