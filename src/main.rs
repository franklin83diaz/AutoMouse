slint::include_modules!();
use device_query::{DeviceQuery, DeviceState};
use slint::{ComponentHandle, LogicalPosition};


fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
  
    // main_window.show();
    let app_handle = main_window.as_weak();
    main_window.on_move_windows(move || {
        if let Some(main_window) = app_handle.upgrade() {
            let device_state = DeviceState::new();
            let mouse = device_state.get_mouse();
            let (x, y) = mouse.coords;
            let x =x -15;
            let y = y-15;

            main_window
                .window()
                .set_position(LogicalPosition::new(x as f32, y as f32));
        }
    });

    main_window.run()
}
