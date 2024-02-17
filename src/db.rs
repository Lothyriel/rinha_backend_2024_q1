use rusqlite::Connection;

use crate::models::IoError;

pub fn get_connection() -> Result<Connection, IoError> {
    const DATABASE_FILE: &str = "rinha.db";
    Ok(Connection::open(DATABASE_FILE)?)
}

pub fn seed_data() -> Result<(), IoError> {
    let conn = get_connection()?;

    let clients_limits = [
        (1, 100000),
        (2, 80000),
        (3, 1000000),
        (4, 10000000),
        (5, 500000),
    ];

    for (id, limit) in clients_limits {
        conn.execute(
            "INSERT INTO clients (id, limit, balance) VALUES (?, ?, ?)",
            (id, limit, 0),
        )?;
    }

    Ok(())
}

pub fn grant_database_tables() -> Result<(), IoError> {
    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE account (
             id INTEGER PRIMARY KEY,
             limit INTEGER NOT NULL,
             balance INTEGER NOT NULL
        );",
        (),
    )?;

    conn.execute(
        "CREATE TABLE transactions (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             client_id INTEGER NOT NULL,
             value REAL NOT NULL,
             type TEXT NOT NULL,
             description TEXT NOT NULL,
             date DATETIME NOT NULL
        );",
        (),
    )?;

    Ok(())
}