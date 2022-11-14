use rusqlite::{Connection, Result, Error};

// Stack module
pub mod stack;

// Card module
pub mod card;

// Return connection
pub fn init(path: &str) -> Result<Connection, rusqlite::Error> {
    let conn = connect_db(path)?;
    Ok(conn)
}

// Connect to db if exists or create db with tables
fn connect_db(path: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    conn.execute("CREATE TABLE IF NOT EXISTS stack (id    INTEGER PRIMARY KEY, name    TEXT NOT NULL)", ())?;
    conn.execute("CREATE TABLE IF NOT EXISTS card (id    INTEGER PRIMARY KEY, title    TEXT NOT NULL, text    TEXT NOT NULL, stack_id   INTEGER NOT NULL, FOREIGN KEY(stack_id) REFERENCES stack(id))", ())?;
    Ok(conn)
}
