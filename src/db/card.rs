use rusqlite::Connection;

// Card Struct
pub struct Card {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub stack_id: i32,
}

// Add card
pub fn add(conn: &Connection, stack_id: i32, title: String, text: String) {
    let card = Card {
        id: 0,
        title,
        text,
        stack_id,
    };

    conn.execute(
        "INSERT INTO card (title, text, stack_id) VALUES (?1, ?2, ?3)",
        (&card.title, &card.text, &card.stack_id)
    ).unwrap();
}

// List cards
pub fn list(conn: &Connection, stack_id: i32) -> Vec<Card> {
    let mut raw_cards = conn.prepare(format!("SELECT * FROM card WHERE stack_id={}", stack_id).as_str()).unwrap();
    let card_result = raw_cards.query_map([], |row| {
        Ok(Card {
            id: row.get(0)?,
            title: row.get(1)?,
            text: row.get(2)?,
            stack_id: row.get(3)?,
        })
    }).unwrap();
    let mut cards = Vec::new();
    for card in card_result {
        let i_unwraped = card.unwrap();
        let i: Card = Card {
            id: i_unwraped.id,
            title: i_unwraped.title,
            text: i_unwraped.text,
            stack_id: i_unwraped.stack_id,
        };
        cards.push(i);
    }

    cards
}

// Delete card
pub fn delete(conn: &Connection, id: i32) {
    let card = Card {
        id,
        text: String::new(),
        title: String::new(),
        stack_id: 0,
    };

    conn.execute(
        "DELETE FROM card WHERE id=(?1)",
        (&card.id, )
    )
        .unwrap(); 
}

// Edit card
pub fn edit(conn: &Connection, id: i32, title: String, text: String) {
    let card = Card {
        id,
        title,
        text,
        stack_id: 0,
    };

    conn.execute(
        "UPDATE card SET title=(?1), text=(?2) WHERE id=(?3)",
        (&card.title, &card.text, &card.id)
    )
        .unwrap();
}
