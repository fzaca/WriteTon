use tabled::{
    settings::{
        object,
        style::{HorizontalLine, VerticalLine},
        Color, Style, Width,
    },
    Table,
};

use crate::database;
use crate::models::Note;

pub fn exec() {
    match get_notes() {
        Ok(notes) => {
            if notes.is_empty() {
                println!("There are no notes.");
            } else {
                print_notes(notes);
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

fn print_notes(notes: Vec<Note>) {
    let mut table = Table::new(notes);

    let theme = Style::modern() // FIXME: Separate in utils file this theme with colors
        .horizontals([(1, HorizontalLine::inherit(Style::modern()))])
        .verticals([(1, VerticalLine::inherit(Style::modern()))])
        .remove_horizontal()
        .remove_vertical();

    table
        .with(theme)
        .modify(object::Columns::single(0), Color::FG_BRIGHT_RED)
        .modify(object::Columns::single(1), Color::FG_BRIGHT_CYAN)
        .modify(object::Rows::first(), Color::FG_BRIGHT_BLACK)
        .modify(
            object::Columns::single(1),
            Width::truncate(40).suffix("..."),
        );

    println!("{}", table.to_string());
}
