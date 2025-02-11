
use slint::SharedString;

use super::strucs::{Setting, CONFIG_INSTANCE};

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
    
    let each_return = SharedString::from(n.unwrap().to_string());
    main_window.set_repeat_each(each_return);
}

fn parse_int(input: &str) -> Option<i32> {
    input.trim().parse::<i32>().ok()
}
