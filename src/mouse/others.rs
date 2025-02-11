use slint::{ComponentHandle, LogicalPosition};
use device_query::{DeviceQuery, DeviceState};
use crate::config::data::{CONFIG_INSTANCE, Setting};
use crate::config::set::repeat_each;

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
    });

    main_window.on_record(move || {
        conf.set_recoding(true);
    });

    let main_window_set_repeat_each = main_window.clone_strong();
    main_window.on_set_repeat_each(move |v| {
        repeat_each(v, &main_window_set_repeat_each);
    });
}
