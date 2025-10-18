use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        Ok(Database {
            conn: Mutex::new(conn),
        })
    }
    
    pub fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        
        // Run migrations
        conn.execute_batch(include_str!("../migrations/V1__initial_schema.sql"))?;
        
        Ok(())
    }
}

pub fn get_db_path(app: &AppHandle) -> PathBuf {
    let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");
    std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
    app_data_dir.join("axiomhive_notes.db")
}
