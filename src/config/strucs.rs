use std::sync::OnceLock;
use std::sync::Mutex;
use crossbeam::channel;

use channel::{Sender, Receiver};

pub struct Communication {
   pub tx: Sender<i32>,
   pub rx: Receiver<i32>,
}
//new
impl Communication {
    pub fn new() -> Self {
        let (tx, rx) = channel::bounded::<i32>(1);
        Communication { tx, rx }
    }
}

pub static CON_INSTANCE: OnceLock<Communication> = OnceLock::new();

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

    pub  fn set_recoding(&self, recoding: bool) {
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

    pub  fn set_repeat(&self, repeat: bool) {
        if cfg!(debug_assertions) {
        println!("set_repeat: {}", repeat);
        }
        let mut data = self.repeat.lock().unwrap();
        *data = repeat;                            
    }

    pub fn set_repeat_each(&self, repeat_each: i32) {
        if cfg!(debug_assertions) {
        println!("set_repeat_each: {}", repeat_each);
        }

        let mut data = self.repeat_each.lock().unwrap(); 
        *data = repeat_each; 
     
    }

    pub fn set_key_stop(&self, key_stop: char) {
        if cfg!(debug_assertions) {
        println!("set_key_stop: {}", key_stop);
        }
       let mut data = self.key_stop.lock().unwrap();
        *data = key_stop;
    }

    pub fn set_auto_stop(&self, auto_stop: bool) {
        if cfg!(debug_assertions) {
        println!("set_auto_stop: {}", auto_stop);
        }
        let mut data = self.auto_stop.lock().unwrap();
        *data = auto_stop;
    }

    pub fn set_auto_stop_clicks(&self, auto_stop_clicks: i32) {
        if cfg!(debug_assertions) {
        println!("set_auto_stop_clicks: {}", auto_stop_clicks);
        }
        let mut data = self.auto_stop_clicks.lock().unwrap();
        *data = auto_stop_clicks;
    }

}



pub static CONFIG_INSTANCE: OnceLock<Setting> = OnceLock::new();