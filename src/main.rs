//#![windows_subsystem = "windows"]
slint::include_modules!();
mod app;
mod config;
mod crud;
mod model;
mod state;

use model::mouse::{MouseEventList, MOUSE_EVENT_LIST};
use slint::ComponentHandle;
use state::global::{RecodingMetaData, RECODING_META_DATA};

fn main() -> Result<(), slint::PlatformError> {
    // Sync Config db
    crud::sql::sync_config_from_db();

    //Main Window
    let main_window = MainWindow::new()?;

    // Run All Threads
    app::threads::run_all_threads(&main_window);

    // Action Bar, recording, move and Close
    app::actions_ui::action_bar(&main_window);

    // Sync UI
    // The copy of the config running in memory is used for fast validation actions in process events
    app::actions_ui::sync_ui(&main_window);

    // Init mouse_event_list
    let _ = MOUSE_EVENT_LIST.get_or_init(MouseEventList::default);
    // init state
    let _ = RECODING_META_DATA.get_or_init(RecodingMetaData::default);

    app::actions_ui::sync_ui_list_macros_from_db(&main_window);

    main_window.run()
}
