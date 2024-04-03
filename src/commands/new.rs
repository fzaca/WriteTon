use crate::database;

pub fn exec(content: String) {
    match database::get_conn() {
        Ok(_) => {
            println!("{content}");
        }
        Err(err) => {
            eprintln!("Error obtaining connection to the database: {:?}", err);
        }
    }
}
