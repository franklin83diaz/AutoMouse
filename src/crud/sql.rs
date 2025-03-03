use rusqlite::{Connection, Result};

use crate::config::data::{ Setting, CONFIG_INSTANCE};
use crate::model::mouse::{MOUSE_EVENT_LIST, MouseEvent, MouseAction, MouseButton};

fn connect() -> Result<Connection> {
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
    conf.get_auto_stop();
    
   if conf.get_auto_stop_clicks() == 0 {
        conf.set_auto_stop_clicks(1);
    }

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

    let stmt_result = conn.prepare("SELECT * FROM config");

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
        milliseconds_runing INTEGER
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
    let mut conn = connect().unwrap();
    let milliseconds_runing= mouse_event_list.get_milliseconds_running();
    let name= mouse_event_list.get_name();

    let result = conn.execute(
        "INSERT INTO mouse_event_list (name, milliseconds_runing) VALUES (?, ?)",
        rusqlite::params![name, milliseconds_runing],
    );

    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            if cfg!(debug_assertions) {
                println!("Mouse macro set successfully");
            }

            let list = mouse_event_list.mouse_events.lock().unwrap();
            let query = format!("INSERT INTO mouse_event (id_list, action, button, time, x, y) VALUES (?, ?, ?, ?, ?, ?)");
            let mut _tx = conn.transaction();
           

            list.iter().for_each(|mouse_event| {
                let (action, button, time_milliseconds, x, y) = mouse_event.get_tuple();

                // conver time_milliseconds to i64
                let time_milliseconds = time_milliseconds as i64;

                // let _ = conn.execute(
                //     "INSERT INTO mouse_event (id_list, action, button, time, x, y) VALUES (?, ?, ?, ?, ?, ?)",
                //     rusqlite::params![id, action as i32, button as i32, time_milliseconds, x, y],
                // );

               let _= _tx.as_mut().unwrap().execute(&query, rusqlite::params![id, action as i32, button as i32, time_milliseconds, x, y]);
                
            });

            _tx.unwrap().commit().unwrap();
        
            

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

pub fn get_mouse_macro_list(id: i32) -> Vec<MouseEvent> {
    let conn = connect().unwrap();
    let mut stmt = conn.prepare("SELECT action, button, time, x, y FROM mouse_event WHERE id_list = ? ORDER BY time ASC").unwrap();

    let mouse_event_list_iter = stmt.query_map(rusqlite::params![id], |row| {
        let  action: MouseAction = match row.get::<_, i32>(0).unwrap() {
            1 => MouseAction::Move,
            2 => MouseAction::Press,
            3 => MouseAction::Release,
            4 => MouseAction::Scroll,
            _ => MouseAction::Unknown,
        };

        let botton: MouseButton = match row.get::<_, i32>(1).unwrap() {
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            _ => MouseButton::Unknown,
        };
    
        Ok(MouseEvent::new(action, botton, row.get("time").unwrap(), row.get("x").unwrap(), row.get("y").unwrap()))
    }).unwrap();


    let mut mouse_event_list = vec![];
    for mouse_event in mouse_event_list_iter {
        mouse_event_list.push(mouse_event.unwrap());
    }
    return mouse_event_list;
}


pub fn get_mouse_macros() -> Vec<(i32, String, i32)> {
    let conn = connect().unwrap();

    //Create if not exists
    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS mouse_event_list (
        id INTEGER PRIMARY KEY,
        name TEXT,
        milliseconds_runing INTEGER
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
 

    let mut stmt = conn.prepare("SELECT id, name, milliseconds_runing FROM mouse_event_list").unwrap();

    let mouse_event_list_iter = stmt.query_map([], |row| {
        Ok((row.get("id").unwrap(), row.get("name").unwrap(), row.get("milliseconds_runing").unwrap()))
    }).unwrap();

    let mut mouse_event_list = vec![];
    for mouse_event in mouse_event_list_iter {
        mouse_event_list.push(mouse_event.unwrap());
    }
    return mouse_event_list;
}

pub fn delete_mouse_macro(id: i32) {
    let conn = connect().unwrap();
    conn.execute("DELETE FROM mouse_event WHERE id_list = ?", rusqlite::params![id]).unwrap();
    conn.execute("DELETE FROM mouse_event_list WHERE id = ?", rusqlite::params![id]).unwrap();
    conn.close().unwrap();
}
 