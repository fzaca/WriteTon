use crate::database;
use crate::utils::random_id;

pub fn exec(content: &String) {
    match create_note(&content) {
        Ok(_) => println!("Note created successfully"),
        Err(err) => eprintln!("Error created note: {:?}", err),
    }
}

fn create_note(content: &String) -> Result<(), rusqlite::Error> {
    let conn = database::get_conn()?;
    conn.execute(
        "INSERT INTO note (id, content) VALUES (?1, ?2)",
        (random_id::generate_random_id(), content),
    )?;
    Ok(())
}
