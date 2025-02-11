slint::include_modules!();
mod config;
mod mouse;
mod database;


use slint::ComponentHandle;


fn main() -> Result<(), slint::PlatformError> {
    // Sync Config db
    database::sql::sync_config_from_db();

    //Main Window
    let main_window = MainWindow::new()?;

    // Run All Threads
    mouse::threads::run_all_threads(&main_window);
    

    // Action Bar, recording, move and Close
    mouse::others::action_bar(&main_window);

    // Sync UI
    mouse::others::sync_ui(&main_window);
    
    main_window.run()
}
