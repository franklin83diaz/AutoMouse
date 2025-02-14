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
    // Recoding
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

    // Repeat
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

    // Repeat Each
    pub fn set_repeat_each(&self, repeat_each: i32) {
        if cfg!(debug_assertions) {
            println!("set_repeat_each: {}", repeat_each);
        }

        let mut data = self.repeat_each.lock().unwrap();
        *data = repeat_each;
    }

    pub fn get_repeat_each(&self) -> i32 {
        let data = self.repeat_each.lock().unwrap();
        *data
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
pub fn map_key(key: char) -> (char, rdev::Key) {
    match key {
        'a' => ('A', rdev::Key::KeyA),
        'b' => ('B', rdev::Key::KeyB),
        'c' => ('C', rdev::Key::KeyC),
        'd' => ('D', rdev::Key::KeyD),
        'e' => ('E', rdev::Key::KeyE),
        'f' => ('F', rdev::Key::KeyF),
        'g' => ('G', rdev::Key::KeyG),
        'h' => ('H', rdev::Key::KeyH),
        'i' => ('I', rdev::Key::KeyI),
        'j' => ('J', rdev::Key::KeyJ),
        'k' => ('K', rdev::Key::KeyK),
        'l' => ('L', rdev::Key::KeyL),
        'm' => ('M', rdev::Key::KeyM),
        'n' => ('N', rdev::Key::KeyN),
        'o' => ('O', rdev::Key::KeyO),
        'p' => ('P', rdev::Key::KeyP),
        'q' => ('Q', rdev::Key::KeyQ),
        'r' => ('R', rdev::Key::KeyR),
        's' => ('S', rdev::Key::KeyS),
        't' => ('T', rdev::Key::KeyT),
        'u' => ('U', rdev::Key::KeyU),
        'v' => ('V', rdev::Key::KeyV),
        'w' => ('W', rdev::Key::KeyW),
        'x' => ('X', rdev::Key::KeyX),
        'y' => ('Y', rdev::Key::KeyY),
        'z' => ('Z', rdev::Key::KeyZ),
        _ => ('Q', rdev::Key::KeyQ),
    }
}

#[derive(Debug)]
pub struct MouseTracker {
    time: i32,
    left_click: bool,
    right_click: bool,
    x: i32,
    y: i32,
}

impl MouseTracker {
    pub fn new(time: i32, left_click: bool, right_click: bool, x: i32, y: i32) -> Self {
        MouseTracker {
            time,
            left_click,
            right_click,
            x,
            y,
        }
    }
}

#[derive(Default)]
pub struct MouseTrackerList {
    name: Mutex<String>,
    start_time_unix: Mutex<i32>,
    end_time_unix: Mutex<i32>,
    list: Mutex<Vec<MouseTracker>>,
}

impl std::fmt::Display for MouseTrackerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let name = self.name.lock().unwrap();
        let start_time_unix = self.start_time_unix.lock().unwrap();
        let end_time_unix = self.end_time_unix.lock().unwrap();
        let list = self.list.lock().unwrap();

        write!(f, "name: {}, start_time_unix: {}, end_time_unix: {}, list: {:?}", *name, *end_time_unix, *start_time_unix, *list)
    }
}


impl MouseTrackerList {

    pub fn add(&self, mouse_tracker: MouseTracker) {
        if cfg!(debug_assertions) {
            println!("add mouse tracker");
        }
        let mut data = self.list.lock().unwrap();
        data.push(mouse_tracker);
    }

    pub fn set_name(&self, name: String) {
        if cfg!(debug_assertions) {
            println!("set_name: {}", name);
        }
        let mut data = self.name.lock().unwrap();
        *data = name;
    }

    pub fn set_start_time_unix(&self, time: i32) {
        if cfg!(debug_assertions) {
            println!("set_total_time_show: {}", time);
        }
        let mut data = self.start_time_unix.lock().unwrap();
        *data = time;
    }

    pub fn set_end_time_unix(&self, time: i32) {
        if cfg!(debug_assertions) {
            println!("set_total_time_show: {}", time);
        }
        let mut data = self.end_time_unix.lock().unwrap();
        *data = time;
    }
    

}



pub static MOUSE_TRACKER_LIST: OnceLock<MouseTrackerList> = OnceLock::new();
pub static CON_INSTANCE: OnceLock<Communication> = OnceLock::new();
pub static CONFIG_INSTANCE: OnceLock<Setting> = OnceLock::new();
