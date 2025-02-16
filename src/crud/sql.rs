use crate::config::data::{ Setting, CONFIG_INSTANCE};
use crate::model::mouse::{MOUSE_EVENT_LIST, mouse_event_list};
use rusqlite::{Connection, Result};

fn connect() -> Result<Connection> {
    // Cambia el tipo de retorno a Result<Connection>
    let conn = Connection::open("automouse.db")?;
    conn.execute_batch("SELECT 1;")?;

    if cfg!(debug_assertions) {
        println!("Database open successfully.");
    }
    Ok(conn)
}

pub fn set_config() {
    let conn = connect().unwrap();
    //Drop the table if it exists
    conn.execute("DROP TABLE IF EXISTS config", []).unwrap();
    conn.execute(
        "CREATE TABLE config (
            id INTEGER PRIMARY KEY,
            recoding BOOLEAN,
            repeat BOOLEAN,
            repeat_each INTEGER,
            key_stop TEXT,
            auto_stop BOOLEAN,
            auto_stop_clicks INTEGER
        )",
        [],
    )
    .unwrap();

    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);
    conn.execute(
        "INSERT INTO config (recoding, repeat, repeat_each, key_stop, auto_stop, auto_stop_clicks) VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            conf.get_recoding(),
            conf.get_repeat(),
            conf.get_repeat_each(),
            conf.get_key_stop().to_string(),
            conf.get_auto_stop(),
            conf.get_auto_stop_clicks()
        ],
    ).unwrap();

    conn.close().unwrap();
}

pub fn update_config() {
    let conn = connect().unwrap();
    let conf = CONFIG_INSTANCE.get_or_init(Setting::default);
    if cfg!(debug_assertions) {
        println!("Update config");
        println!("recoding: {}", conf.get_recoding());
        println!("repeat: {}", conf.get_repeat());
        println!("repeat_each: {}", conf.get_repeat_each());
        println!("key_stop: {}", conf.get_key_stop());
        println!("auto_stop: {}", conf.get_auto_stop());
        println!("auto_stop_clicks: {}", conf.get_auto_stop_clicks());
    }

    conn.execute(
        "UPDATE config SET recoding = ?, repeat = ?, repeat_each = ?, key_stop = ?, auto_stop = ?, auto_stop_clicks = ? WHERE id = 1",
        rusqlite::params![
            conf.get_recoding(),
            conf.get_repeat(),
            conf.get_repeat_each(),
            conf.get_key_stop().to_string(),
            conf.get_auto_stop(),
            conf.get_auto_stop_clicks()
        ],
    ).unwrap();

    conn.close().unwrap();
}

pub fn sync_config_from_db() {
    let conn = connect().unwrap();

    let mut stmt_result = conn.prepare("SELECT * FROM config");

    match stmt_result {
        Ok(mut stmt) => {
            let config_iter = stmt
                .query_map([], |row| {
                    Ok((
                        row.get::<_, i32>(0)?,
                        row.get::<_, bool>(1)?,
                        row.get::<_, bool>(2)?,
                        row.get::<_, i32>(3)?,
                        row.get::<_, String>(4)?.chars().next().unwrap(),
                        row.get::<_, bool>(5)?,
                        row.get::<_, i32>(6)?,
                    ))
                })
                .unwrap();

            let conf = CONFIG_INSTANCE.get_or_init(Setting::default);
            for config in config_iter {
                let (_, recoding, repeat, repeat_each, key_stop, auto_stop, auto_stop_clicks) =
                    config.unwrap();
                conf.set_recoding(recoding);
                conf.set_repeat(repeat);
                conf.set_repeat_each(repeat_each);
                conf.set_key_stop(key_stop);
                conf.set_auto_stop(auto_stop);
                conf.set_auto_stop_clicks(auto_stop_clicks);
            }
        }
        Err(_) => {
            if cfg!(debug_assertions) {
                println!("create table config and insert default values");
            }
            crate::crud::sql::set_config();
            return;
        }
    }
}


//Mouse Macros
pub fn save_mouse_macro() {
    let conn = connect().unwrap();

    //Create if not exists
    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS mouse_event_list (
        id INTEGER PRIMARY KEY,
        name TEXT,
        miliseconds_runing INTEGER
    )",
        [],
    );

    match result {
        Ok(_) => {
            if cfg!(debug_assertions) {
                println!("Table mouse_event_list created successfully");
            }
        }
        Err(err) => {
            if cfg!(debug_assertions) {
                println!("Error creating table mouse_event_list: {}", err);
            }
        }
    }
 
    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS mouse_event (
        id INTEGER PRIMARY KEY,
        id_list INTEGER,
        action INTEGER,
        button INTEGER,
        time INTEGER,
        x INTEGER,
        y INTEGER
    )",
        [],
    );

    match result {
        Ok(_) => {
            if cfg!(debug_assertions) {
                println!("Table mouse_tracker created successfully");
            }
        }
        Err(err) => {
            if cfg!(debug_assertions) {
                println!("Error creating table mouse_tracker: {}", err);
            }
        }
    }

    let mouse_event_list = MOUSE_EVENT_LIST.get().unwrap();
    let conn = connect().unwrap();
    let miliseconds_runing= mouse_event_list.get_miliseconds_runing();
    let name= mouse_event_list.get_name();

    let result = conn.execute(
        "INSERT INTO mouse_event_list (name, miliseconds_runing) VALUES (?, ?)",
        rusqlite::params![name, miliseconds_runing],
    );

    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            if cfg!(debug_assertions) {
                println!("Mouse macro set successfully");
            }

            let list = mouse_event_list.mouse_events.lock().unwrap();
            list.iter().for_each(|mouse_event| {
                let (action, button, time_milliseconds, x, y) = mouse_event.get_tuple();

                // conver time_milliseconds to i64
                let time_milliseconds = time_milliseconds as i64;

                let _ = conn.execute(
                    "INSERT INTO mouse_event (id_list, action, button, time, x, y) VALUES (?, ?, ?, ?, ?, ?)",
                    rusqlite::params![id, action, button, time_milliseconds, x, y],
                );

            });

            if cfg!(debug_assertions) {
                println!("id: {}", id );
            }
        }
        Err(err) => {
            if cfg!(debug_assertions) {
                println!("Error setting mouse macro: {}", err);
            }
        }
    }

    conn.close().unwrap();
}
