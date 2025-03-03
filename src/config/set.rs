
use slint::SharedString;

use super::data::{Setting, CONFIG_INSTANCE, map_key};
// Repeat
// No need set function, is same type bool

// Repeat Each
pub fn repeat_each(v: SharedString, main_window:&crate::slint_generatedMainWindow::MainWindow  ){
    let  conf = CONFIG_INSTANCE.get_or_init(Setting::default);
    
    if v.is_empty() {
            conf.set_repeat_each(1);
        return;
    }

    let mut n = parse_int(v.as_str());

    if n.unwrap() > 10 {
        n = Some(10);
    }

    conf.set_repeat_each(n.unwrap());
    
    let value = SharedString::from(n.unwrap().to_string());
    main_window.set_repeat_each(value);
}

// Key Stop
pub fn key_stop(v: SharedString, main_window:&crate::slint_generatedMainWindow::MainWindow  ){
    let  conf = CONFIG_INSTANCE.get();
    
    if v.is_empty() {
            conf.unwrap().set_key_stop('q');
        return;
    }

    let mut k = v.chars().next().unwrap();

    // convert to lowercase
    k = k.to_lowercase().next().unwrap();

    conf.unwrap().set_key_stop(k);
    
    let value = SharedString::from(map_key(k).0.to_string());
    main_window.set_key_stop(value);
}


pub fn auto_stop_clicks(v: SharedString, main_window:&crate::slint_generatedMainWindow::MainWindow  ){
    let  conf = CONFIG_INSTANCE.get();
    
    if v.is_empty() || v == "0" {
            conf.unwrap().set_auto_stop_clicks(1);
        return;
    }

    let mut n = parse_int(v.as_str());

    if n.unwrap() > 10 {
        n = Some(10);
    }

    conf.unwrap().set_auto_stop_clicks(n.unwrap());
    
    let value = SharedString::from(n.unwrap().to_string());
    main_window.set_auto_stop_clicks(value);
}

fn parse_int(input: &str) -> Option<i32> {
    input.trim().parse::<i32>().ok()
}
