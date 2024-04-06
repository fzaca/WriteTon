use colored::Colorize;

use crate::database;
use crate::models::Note;
use crate::utils::truncate::truncate_string;

pub fn exec() {
    match get_notes() {
        Ok(notes) => {
            if notes.is_empty() {
                println!("No hay ninguna nota.");
            } else {
                for note in notes {
                    let truncated_content = truncate_string(&note.content, 50, true);
                    println!(" {} {}", note.id.bright_red(), truncated_content);
                }
            }
        }
        Err(err) => eprintln!("Error getting the list of notes: {:?}", err),
    }
}

fn get_notes() -> Result<Vec<Note>, rusqlite::Error> {
    let conn = database::get_conn()?;
    let mut stmt = conn.prepare("SELECT id, content FROM note")?;
    let note_iter = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            content: row.get(1)?,
        })
    })?;

    let mut notes = vec![];
    for note in note_iter {
        notes.push(note?);
    }

    Ok(notes)
}
