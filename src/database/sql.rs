use rusqlite::{Connection, Result};
use crate::config::data::{CONFIG_INSTANCE, Setting};

fn connect() -> Result<Connection> { // Cambia el tipo de retorno a Result<Connection>
    let conn = Connection::open("automouse.db")?;
    conn.execute_batch("SELECT 1;")?;

    if cfg!(debug_assertions) {
        println!("Database open successfully.");
    } 
    Ok(conn) 
}

pub fn set_config(){
    let conn= connect().unwrap();
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
    ).unwrap();

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

pub fn get_config(){

    let conn = connect().unwrap();
    let mut stmt_result = conn.prepare("SELECT * FROM config");

    match stmt_result {
        Ok(mut stmt) => {
            let config_iter = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, bool>(1)?,
                    row.get::<_, bool>(2)?,
                    row.get::<_, i32>(3)?,
                    row.get::<_, String>(4)?.chars().next().unwrap(),
                    row.get::<_, bool>(5)?,
                    row.get::<_, i32>(6)?
                ))
            }).unwrap();

            for config in config_iter {
                println!("Config: {:?}", config.unwrap());
            }
        },
        Err(_) => {
            if cfg!(debug_assertions) {
                println!("create table config and insert default values");
            }
            crate::database::sql::set_config();
            return;
        }
    }

    



    
}