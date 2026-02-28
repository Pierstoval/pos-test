use rusqlite::Connection;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;

/// Wrapper around a SQLite connection so it can be managed as Tauri state.
/// The Mutex ensures that concurrent command invocations do not race on the
/// single connection.
pub struct DbState {
    pub conn: Mutex<Connection>,
}

#[cfg(test)]
pub fn init_db_in_memory() -> DbState {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory database");
    conn.execute_batch("PRAGMA foreign_keys=ON;")
        .expect("Failed to enable foreign keys");
    create_tables(&conn).expect("Failed to create tables");
    DbState {
        conn: Mutex::new(conn),
    }
}

/// Opens (or creates) the SQLite database file inside the app's data directory
/// and returns a `DbState` ready to be managed by Tauri.
///
/// The database file is named `pos.db` and lives in the directory returned by
/// `app_handle.path().app_data_dir()`.
pub fn init_db(app_handle: &AppHandle) -> Result<DbState, String> {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data dir: {e}"))?;

    // Ensure the directory exists.
    std::fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create app data dir: {e}"))?;

    let db_path = data_dir.join("pos.db");

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database at {}: {e}", db_path.display()))?;

    // Enable WAL mode for better concurrent read performance.
    conn.execute_batch("PRAGMA journal_mode=WAL;")
        .map_err(|e| format!("Failed to set WAL mode: {e}"))?;

    // Enable foreign keys.
    conn.execute_batch("PRAGMA foreign_keys=ON;")
        .map_err(|e| format!("Failed to enable foreign keys: {e}"))?;

    create_tables(&conn)?;

    Ok(DbState {
        conn: Mutex::new(conn),
    })
}

/// Creates the application tables if they do not already exist.
pub fn create_tables(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS products (
            id        TEXT PRIMARY KEY NOT NULL,
            name      TEXT NOT NULL,
            price     INTEGER NOT NULL,
            category  TEXT NOT NULL CHECK (category IN ('snack', 'soft_drink', 'alcohol', 'sweets')),
            available INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS orders (
            id              TEXT PRIMARY KEY NOT NULL,
            created_at      TEXT NOT NULL,
            total           INTEGER NOT NULL,
            payment_method  TEXT NOT NULL CHECK (payment_method IN ('cash', 'card'))
        );

        CREATE TABLE IF NOT EXISTS order_items (
            id            TEXT PRIMARY KEY NOT NULL,
            order_id      TEXT NOT NULL,
            product_id    TEXT NOT NULL,
            product_name  TEXT NOT NULL,
            unit_price    INTEGER NOT NULL,
            quantity      INTEGER NOT NULL,
            total         INTEGER NOT NULL,
            FOREIGN KEY (order_id)   REFERENCES orders   (id),
            FOREIGN KEY (product_id) REFERENCES products (id)
        );

        CREATE INDEX IF NOT EXISTS idx_order_items_order_id
            ON order_items (order_id);
        ",
    )
    .map_err(|e| format!("Failed to create tables: {e}"))?;

    Ok(())
}
