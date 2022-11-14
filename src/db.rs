use rusqlite::{Connection, Result, Error};

// Stack module
pub mod stack;

// Return connection
pub fn init(path: &str) -> Result<Connection, rusqlite::Error> {
    let conn = connect_db(path)?;
    Ok(conn)
}

// Connect to db if exists or create db with tables
fn connect_db(path: &str) -> Result<Connection, Error> {
    let conn = Connection::open(path)?;
    conn.execute("CREATE TABLE IF NOT EXISTS stack (id    INTEGER PRIMARY KEY, name    TEXT NOT NULL)", ())?;
    Ok(conn)
}
