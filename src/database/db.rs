use rusqlite::{params, Connection};


pub fn initialize_database() -> Connection {
 
    let database_url = "./visits.db";

    let conn = Connection::open(database_url).expect("Failed to connect to database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS visits (
            device_id VARCHAR(36),
            visit_id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        )",
        params![],
    )
    .expect("Failed to create table");

    conn
}
