use rusqlite::Connection;

// Stack struct
pub struct Stack {
    pub id: i32,
    pub name: String,
}

// Get all stacks
pub fn get_all(conn: &Connection) -> Vec<Stack> {
    let mut raw_stacks = conn.prepare("SELECT * FROM stack").unwrap();
    let stacks_result = raw_stacks.query_map([], |row| {
        Ok(Stack {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).unwrap();
    let mut stacks = Vec::new();
    for stack in stacks_result {
        let i_unwraped = stack.unwrap();
        let i: Stack = Stack {
            id: i_unwraped.id,
            name: i_unwraped.name,
        };
        stacks.push(i);
    }

    stacks
}

// Add stack 
pub fn add(conn: &Connection, name: String) {
    let stack = Stack {
        name: name,
        id: 0
    };

    conn.execute(
        "INSERT INTO stack (name) VALUES (?1)",
        (&stack.name, )
    ).unwrap();
}

// Delete stack 
pub fn delete(conn: &Connection, id: i32) {
    let stack = Stack {
        name: String::from(""),
        id: id,
    };

    conn.execute(
        "DELETE FROM stack WHERE id=(?1)",
        (&stack.id, )
    )
        .unwrap(); 
}
