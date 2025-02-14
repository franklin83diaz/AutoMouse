slint::include_modules!();
mod app;
mod config;
mod model;

use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    // Sync Config db
    model::sql::sync_config_from_db();

    //Main Window
    let main_window = MainWindow::new()?;

    // Run All Threads
    app::threads::run_all_threads(&main_window);

    // Action Bar, recording, move and Close
    app::actions_ui::action_bar(&main_window);

    // Sync UI
    // The copy of the config running in memory is used for fast validation actions in process events
    app::actions_ui::sync_ui(&main_window);

    main_window.run()
}
