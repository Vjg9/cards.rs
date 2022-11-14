use rusqlite::Connection;

// Card Struct
pub struct Card {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub stack_id: i32,
}
