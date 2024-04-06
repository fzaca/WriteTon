use rusqlite::{Connection, Result};

pub fn get_conn() -> Result<Connection> {
    let project_dir = ".local/share/writeton";
    let home_dir = dirs::home_dir().expect("Error in get user home");
    let conn = Connection::open(&format!(
        "{}/{}{}",
        home_dir.display(),
        project_dir,
        "/data.db"
    ))?;
    Ok(conn)
}

pub fn ensure_database_created() -> Result<()> {
    match get_conn() {
        Ok(conn) => {
            conn.execute(
                "CREATE TABLE note (
                    id INTEGER PRIMARY KEY,
                    content TEXT
                )",
                (),
            )?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
