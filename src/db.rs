use async_sqlite::rusqlite::{Connection, Error};

pub fn seed_data(conn: &Connection) -> Result<(), Error> {
    let clients_limits = [
        (1, 100000),
        (2, 80000),
        (3, 1000000),
        (4, 10000000),
        (5, 500000),
    ];

    for (id, limit) in clients_limits {
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
             value REAL NOT NULL,
             type TEXT NOT NULL,
             description TEXT NOT NULL,
             date DATETIME NOT NULL
        );",
        (),
    )?;

    Ok(())
}
