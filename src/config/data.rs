use channel::{Receiver, Sender};
use crossbeam::channel;
use std::sync::Mutex;
use std::sync::OnceLock;


// Communication between threads
pub struct Communication {
    ctr_press: Mutex<bool>,
    pub tx: Sender<i32>,
    pub rx: Receiver<i32>,
}

//new
impl Communication {
    pub fn new() -> Self {
        let (tx, rx) = channel::bounded::<i32>(1);
        let ctr_press = Mutex::new(false);
        Communication { ctr_press, tx, rx }
    }
}

impl Communication {
    pub fn set_ctr_press(&self, ctr_press: bool) {
        if cfg!(debug_assertions) {
            println!("set_ctr_press: {}", ctr_press);
        }
        let mut data = self.ctr_press.lock().unwrap();
        *data = ctr_press;
    }

    pub fn get_ctr_press(&self) -> bool {
        let data = self.ctr_press.lock().unwrap();
        *data
    }
}

// The application Setting
#[derive(Default)]
pub struct Setting {
    recoding: Mutex<bool>,
    repeat: Mutex<bool>,
    repeat_each: Mutex<i32>,
    key_stop: Mutex<char>,
    auto_stop: Mutex<bool>,
    auto_stop_clicks: Mutex<i32>,
}

impl Setting {
    pub fn set_recoding(&self, recoding: bool) {
        if cfg!(debug_assertions) {
            println!("set_recoding: {}", recoding);
        }
        let mut data = self.recoding.lock().unwrap();
        *data = recoding;
    }

    pub fn get_recoding(&self) -> bool {
        let data = self.recoding.lock().unwrap();
        *data
    }

    pub fn set_repeat(&self, repeat: bool) {
        if cfg!(debug_assertions) {
            println!("set_repeat: {}", repeat);
        }
        let mut data = self.repeat.lock().unwrap();
        *data = repeat;
    }

    pub fn get_repeat(&self) -> bool {
        let data = self.repeat.lock().unwrap();
        *data
    }

    pub fn get_repeat_each(&self) -> i32 {
        let data = self.repeat_each.lock().unwrap();
        *data
    }

    pub fn set_repeat_each(&self, repeat_each: i32) {
        if cfg!(debug_assertions) {
            println!("set_repeat_each: {}", repeat_each);
        }

        let mut data = self.repeat_each.lock().unwrap();
        *data = repeat_each;
    }

    pub fn get_key_stop(&self) -> char {
        let data = self.key_stop.lock().unwrap();
        *data
    }

    pub fn set_key_stop(&self, key_stop: char) {
        if cfg!(debug_assertions) {
            println!("set_key_stop: {}", key_stop);
        }
        let mut data = self.key_stop.lock().unwrap();
        *data = key_stop;
    }

    pub fn get_auto_stop(&self) -> bool {
        let data = self.auto_stop.lock().unwrap();
        *data
    }

    pub fn set_auto_stop(&self, auto_stop: bool) {
        if cfg!(debug_assertions) {
            println!("set_auto_stop: {}", auto_stop);
        }
        let mut data = self.auto_stop.lock().unwrap();
        *data = auto_stop;
    }

    pub fn get_auto_stop_clicks(&self) -> i32 {
        let data = self.auto_stop_clicks.lock().unwrap();
        *data
    }

    pub fn set_auto_stop_clicks(&self, auto_stop_clicks: i32) {
        if cfg!(debug_assertions) {
            println!("set_auto_stop_clicks: {}", auto_stop_clicks);
        }
        let mut data = self.auto_stop_clicks.lock().unwrap();
        *data = auto_stop_clicks;
    }
}

//mapping chat to key
// Default to Q
pub fn map_key(key: char) -> Option<rdev::Key> {
    match key {
        'a' => Some(rdev::Key::KeyA),
        'b' => Some(rdev::Key::KeyB),
        'c' => Some(rdev::Key::KeyC),
        'd' => Some(rdev::Key::KeyD),
        'e' => Some(rdev::Key::KeyE),
        'f' => Some(rdev::Key::KeyF),
        'g' => Some(rdev::Key::KeyG),
        'h' => Some(rdev::Key::KeyH),
        'i' => Some(rdev::Key::KeyI),
        'j' => Some(rdev::Key::KeyJ),
        'k' => Some(rdev::Key::KeyK),
        'l' => Some(rdev::Key::KeyL),
        'm' => Some(rdev::Key::KeyM),
        'n' => Some(rdev::Key::KeyN),
        'o' => Some(rdev::Key::KeyO),
        'p' => Some(rdev::Key::KeyP),
        'q' => Some(rdev::Key::KeyQ),
        'r' => Some(rdev::Key::KeyR),
        's' => Some(rdev::Key::KeyS),
        't' => Some(rdev::Key::KeyT),
        'u' => Some(rdev::Key::KeyU),
        'v' => Some(rdev::Key::KeyV),
        'w' => Some(rdev::Key::KeyW),
        'x' => Some(rdev::Key::KeyX),
        'y' => Some(rdev::Key::KeyY),
        'z' => Some(rdev::Key::KeyZ),
        _ => Some(rdev::Key::KeyQ),
    }
}

pub static CON_INSTANCE: OnceLock<Communication> = OnceLock::new();
pub static CONFIG_INSTANCE: OnceLock<Setting> = OnceLock::new();
