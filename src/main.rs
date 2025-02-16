slint::include_modules!();
mod app;
mod config;
mod model;
mod state;
mod crud;

use slint::ComponentHandle;
use rdev::EventType;
use model::mouse::{MOUSE_EVENT_LIST, MouseEventList};
use state::global::{RECODIND_META_DATA, RecodingMetaData};

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
    let _ = RECODIND_META_DATA.get_or_init(RecodingMetaData::default);

    let v1 =crud::sql::get_mouse_macro_list(1);
    let mut temp_time =0;

    for v in v1 {
        temp_time = v.3 - temp_time;
        app::actions::send(&EventType::MouseMove { x: v.4 as f64, y: v.2 as f64 }, temp_time);
        println!("{:?}", v);
    }

    main_window.run()
}
