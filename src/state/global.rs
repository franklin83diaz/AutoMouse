use std::sync::OnceLock;
use std::sync::Mutex;

#[derive(Default)]
pub struct RecodingMetaData {
   start_time: Mutex<i32>,
   ctr_press: Mutex<bool>,
}

impl RecodingMetaData {
    pub fn set_start_time_unix(&self, start_time: i32) {
        let mut data = self.start_time.lock().unwrap();
        *data = start_time;
    }

    pub fn get_start_time_unix(&self) -> i32 {
        let data = self.start_time.lock().unwrap();
        *data
    }

    pub fn set_ctr_press(&self, ctr_press: bool) {
        let mut data = self.ctr_press.lock().unwrap();
        *data = ctr_press;
    }

    pub fn get_ctr_press(&self) -> bool {
        let data = self.ctr_press.lock().unwrap();
        *data
    }
    
}


pub static RECODIND_META_DATA: OnceLock<RecodingMetaData> = OnceLock::new();