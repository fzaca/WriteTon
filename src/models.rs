use tabled::Tabled;

#[derive(Tabled)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub last_modified: String,
}
