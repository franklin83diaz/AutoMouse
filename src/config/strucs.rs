

#[derive(Default)]
pub struct Setting {
    repeat: bool,
    repeat_each: i32,
    key_stop: char,
    auto_stop: bool,
    auto_stop_clicks: i32,
}

impl Setting {

    pub  fn set_repeat(&mut self, repeat: bool) {
        if cfg!(debug_assertions) {
        println!("set_repeat: {}", repeat);
        }
        self.repeat = repeat;
    }

    pub fn set_repeat_each(&mut self, repeat_each: i32) {
        if cfg!(debug_assertions) {
        println!("set_repeat_each: {}", repeat_each);
        }
        self.repeat_each = repeat_each;
    }

    pub fn set_key_stop(&mut self, key_stop: char) {
        if cfg!(debug_assertions) {
        println!("set_key_stop: {}", key_stop);
        }
        self.key_stop = key_stop;
    }

    pub fn set_auto_stop(&mut self, auto_stop: bool) {
        if cfg!(debug_assertions) {
        println!("set_auto_stop: {}", auto_stop);
        }
        self.auto_stop = auto_stop;
    }

    pub fn set_auto_stop_clicks(&mut self, auto_stop_clicks: i32) {
        if cfg!(debug_assertions) {
        println!("set_auto_stop_clicks: {}", auto_stop_clicks);
        }
        self.auto_stop_clicks = auto_stop_clicks;
    }

}