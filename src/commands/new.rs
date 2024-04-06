use crate::database;
use crate::utils::random_id;

pub fn exec(content: &String) {
    match database::get_conn() {
        Ok(conn) => match create_note(&conn, &content) {
            Ok(_) => println!("Note created successfully"),
            Err(err) => eprintln!("Error created note: {:?}", err),
        },
        Err(err) => {
            eprintln!("Error obtaining connection to the database: {:?}", err);
        }
    }
}

fn create_note(conn: &rusqlite::Connection, content: &String) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO note (id, content) VALUES (?1, ?2)",
        (random_id::generate_random_id(), content),
    )?;
    Ok(())
}
