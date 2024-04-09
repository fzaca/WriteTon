use crate::database;

pub fn exec(note_id: &String) {
    match remove_note(note_id) {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                println!("Note remove successfully")
            } else {
                eprintln!("Note ID {} not found", note_id)
            }
        }
        Err(err) => eprintln!("Error remove note: {:?}", err),
    }
}

fn remove_note(note_id: &String) -> Result<usize, rusqlite::Error> {
    let conn = database::get_conn()?;
    conn.execute("DELETE FROM note WHERE id = ?", &[note_id])
}
