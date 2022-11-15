use rusqlite::Connection;

// Card Struct
pub struct Card {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub stack_id: i32,
}

pub fn add(conn: &Connection, stack_id: i32, title: String, text: String) {
    let card = Card {
        id: 0,
        title: title,
        text: text,
        stack_id: stack_id,
    };

    conn.execute(
        "INSERT INTO card (title, text, stack_id) VALUES (?1, ?2, ?3)",
        (&card.title, &card.text, &card.stack_id)
    ).unwrap();
}
