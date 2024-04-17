use rusqlite::{Connection, Result};
use std::path::{Path, PathBuf};

const PROJECT_DIR: &str = ".local/share/writeton";
const DATABASE_FILE: &str = "data.db";

fn get_database_path(home_dir: &Path) -> PathBuf {
    let mut path = home_dir.to_owned();
    path.push(PROJECT_DIR);
    path.push(DATABASE_FILE);
    path
}

pub fn get_conn() -> Result<Connection> {
    let home_dir = dirs::home_dir().expect("Error in get user home");
    let db_path = get_database_path(&home_dir);
    Connection::open(db_path)
}

pub fn ensure_database_created() -> Result<()> {
    let db_path = get_database_path(&dirs::home_dir().expect("Error in get user home"));
    if db_path.exists() {
        return Ok(());
    }

    let conn = get_conn()?;
    conn.execute(
        "CREATE TABLE note (
            id TEXT PRIMARY KEY,
            content TEXT,
            last_modified DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;

    conn.execute(
        "CREATE TRIGGER update_last_modified 
            AFTER UPDATE ON note 
            FOR EACH ROW 
            BEGIN 
                UPDATE note SET last_modified = CURRENT_TIMESTAMP WHERE id = OLD.id; 
            END;",
        (),
    )?;

    Ok(())
}
