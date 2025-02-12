use crate::config::data::{Setting, CONFIG_INSTANCE};
use crate::config::set::repeat_each;
use crate::database;
use device_query::{DeviceQuery, DeviceState};
use slint::{ComponentHandle, LogicalPosition, SharedString};

pub fn action_bar(main_window: &crate::slint_generatedMainWindow::MainWindow) {
    let handle_weak = main_window.as_weak();
    main_window.on_move_windows(move || {
        if let Some(main_window) = handle_weak.upgrade() {
            let device_state = DeviceState::new();
            let mouse = device_state.get_mouse();
            let (x, y) = mouse.coords;
            let x = x - 15;
            let y = y - 15;

            main_window
                .window()
                .set_position(LogicalPosition::new(x as f32, y as f32));
        }
    });

    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);
    main_window.on_set_repeat(move |v| {
        conf.set_repeat(v);
        database::sql::update_config();
    });

    main_window.on_record(move || {
        conf.set_recoding(true);
    });

    let main_window_set_repeat_each = main_window.clone_strong();
    main_window.on_set_repeat_each(move |v| {
        repeat_each(v, &main_window_set_repeat_each);
        database::sql::update_config();
    });
}

pub fn sync_ui(main_window: &crate::slint_generatedMainWindow::MainWindow) {
    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);
    let handle_weak = main_window.as_weak();
    if let Some(main_window) = handle_weak.upgrade() {
        if cfg!(debug_assertions) {
            //println!("repeat: {}", conf.get_repeat());
            println!("repeat_each: {}", conf.get_repeat_each());
        }

        main_window.set_repeat_each(SharedString::from(conf.get_repeat_each().to_string()));
    }
}
