use async_sqlite::rusqlite::{Connection, Error};

pub fn seed_data(conn: &Connection, data: &[(u32, u32)]) -> Result<(), Error> {
    for (id, limit) in data {
        conn.execute(
            "INSERT OR IGNORE INTO clients (id, debit_limit, balance) VALUES (?1, ?2, ?3)",
            (id, limit, 0),
        )?;
    }

    Ok(())
}

pub fn grant_database_tables(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clients (
             id INTEGER PRIMARY KEY,
             debit_limit INTEGER NOT NULL,
             balance INTEGER NOT NULL
        );",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             client_id INTEGER NOT NULL,
             value INTEGER NOT NULL,
             type TEXT NOT NULL,
             description TEXT NOT NULL,
             date DATETIME NOT NULL
        );",
        (),
    )?;

    Ok(())
}
