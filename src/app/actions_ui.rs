use chrono::{Datelike, Local, Timelike};
use device_query::{DeviceQuery, DeviceState};
use slint::{ComponentHandle, LogicalPosition, ModelRc, SharedString, VecModel};
use std::rc::Rc;

use crate::app;
use crate::config::data::{map_key, Setting, CONFIG_INSTANCE, CON_INSTANCE};
use crate::config::set::{auto_stop_clicks, key_stop, repeat_each};
use crate::crud;
use crate::model::mouse::MOUSE_EVENT_LIST;
use crate::slint_generatedMainWindow;
use crate::state::global::RECODING_META_DATA;

pub fn action_bar(main_window: &crate::slint_generatedMainWindow::MainWindow) {
    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);

    // Move Windows
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

    // Minimize Windows
    let handle_weak = main_window.as_weak();
    main_window.on_minimize_requested(move || {
        if let Some(main_window) = handle_weak.upgrade() {
            main_window.window().set_minimized(true);
        }
    });

    main_window.on_close_windows(move || {
        std::process::exit(0);
    });

    // Recording
    main_window.on_record(move || {
        let mel = MOUSE_EVENT_LIST.get().unwrap();
        let metadata = RECODING_META_DATA.get().unwrap();

        let now = Local::now();
        let name = format!(
            "{:02} {:02}, {} {:02}:{:02}:{:02}",
            now.month(),
            now.day(),
            now.year(),
            now.hour(),
            now.minute(),
            now.second()
        );
        mel.set_name(name);
        //set start time in global
        metadata.set_start_time_unix(now.timestamp_millis() as i32);
        metadata.set_clicks(conf.get_auto_stop_clicks() as u8);
        mel.mouse_events.lock().unwrap().clear();
        conf.set_recoding(true);
    });

    // Stop Recording
    main_window.on_stop_record(|| {
        let config = CONFIG_INSTANCE.get().unwrap();
        let mel = MOUSE_EVENT_LIST.get().unwrap();
        let now: chrono::DateTime<Local> = Local::now();
        let rmd = RECODING_META_DATA.get().unwrap();
        let con = CON_INSTANCE.get().unwrap();
        let milliseconds_running = now.timestamp_millis() as i32 - rmd.get_start_time_unix();
        mel.set_milliseconds_running(milliseconds_running);
        config.set_recoding(false);
        crud::sql::save_mouse_macro();
        con.tx.send(1).unwrap();
    });

    // On Replay
    main_window.on_replay(move |id| {
        app::actions::replay(id);
    });

    // List
    let handle_weak = main_window.as_weak();
    main_window.on_refresh_list(move || {
        if let Some(main_window) = handle_weak.upgrade() {
            sync_ui_list_macros_from_db(&main_window);
        }
    });

    // List Windows
    // delete
    let handle_weak = main_window.as_weak();
    main_window.on_delete_macro(move |id| {
        crud::sql::delete_mouse_macro(id);
        sync_ui_list_macros_from_db(&handle_weak.unwrap());
    });

    // Settings
    // No need Action in App

    //// Settings windows
    // Repeat
    main_window.on_set_repeat(move |v| {
        conf.set_repeat(v);
        crud::sql::update_config();
    });

    // Repeat Each
    let main_window_clone = main_window.clone_strong();
    main_window.on_set_repeat_each(move |v| {
        // Need main_window for update the value in the ui when is out range
        repeat_each(v, &main_window_clone);
        crud::sql::update_config();
    });

    // Key Stop
    let main_window_clone = main_window.clone_strong();
    main_window.on_set_key_stop(move |v| {
        key_stop(v, &main_window_clone);
        crud::sql::update_config();
    });

    // Auto Stop
    main_window.on_set_auto_stop(move |v| {
        conf.set_auto_stop(v);
        crud::sql::update_config();
    });

    // Auto Stop Clicks
    let main_window_clone = main_window.clone_strong();
    main_window.on_set_auto_stop_clicks(move |v| {
        // Need main_window for update the value in the ui when is out range
        auto_stop_clicks(v, &main_window_clone);
        crud::sql::update_config();
    });
}

pub fn sync_ui(main_window: &crate::slint_generatedMainWindow::MainWindow) {
    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);
    let handle_weak = main_window.as_weak();
    if let Some(main_window) = handle_weak.upgrade() {
        if cfg!(debug_assertions) {
            println!("repeat: {}", conf.get_repeat());
            println!("repeat_each: {}", conf.get_repeat_each());
            println!("key_stop: {}", conf.get_key_stop());
            println!("auto_stop: {}", conf.get_auto_stop());
            println!("auto_stop_clicks: {}", conf.get_auto_stop_clicks());
        }

        main_window.set_repeat_each(SharedString::from(conf.get_repeat_each().to_string()));
        main_window.set_repeat(conf.get_repeat());
        main_window.set_key_stop(SharedString::from(
            map_key(conf.get_key_stop()).0.to_string(),
        ));
        main_window.set_auto_stop(conf.get_auto_stop());
        main_window
            .set_auto_stop_clicks(SharedString::from(conf.get_auto_stop_clicks().to_string()));
    }
}

pub fn sync_ui_list_macros_from_db(main_window: &crate::slint_generatedMainWindow::MainWindow) {
    let handle_weak = main_window.as_weak();

    let mouse_macros = crud::sql::get_mouse_macros();

    let default_tile = slint_generatedMainWindow::TileData::default();
    let mut initial_vec = vec![];

    for m in mouse_macros {
        let mut tile = default_tile.clone();
        tile.name = SharedString::from(m.1);
        tile.id = m.0;
        tile.time = m.2;

        initial_vec.push(tile);
    }

    let vec_model = VecModel::from(initial_vec);

    let model_rc: ModelRc<slint_generatedMainWindow::TileData> = ModelRc::from(Rc::new(vec_model));

    if let Some(main_window) = handle_weak.upgrade() {
        main_window.set_tile_data(model_rc);
    }
}
