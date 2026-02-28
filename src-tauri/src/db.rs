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
    create_default_data(&conn);
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
    create_default_data(&conn);

    Ok(DbState {
        conn: Mutex::new(conn),
    })
}

/// Creates the application tables if they do not already exist.
pub fn create_tables(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS categories (
            id    TEXT PRIMARY KEY NOT NULL,
            label TEXT NOT NULL,
            color TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS products (
            id          TEXT PRIMARY KEY NOT NULL,
            name        TEXT NOT NULL,
            price       INTEGER NOT NULL,
            category_id TEXT NOT NULL,
            available   INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY (category_id) REFERENCES categories(id)
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

/// Inserts the default categories if they do not already exist.
pub fn create_default_data(conn: &Connection) {
    let defaults = [
        ("snack", "Snack", "#e8a735"),
        ("soft_drink", "Soft drink", "#3b82f6"),
        ("alcohol", "Alcohol", "#8b5cf6"),
        ("sweets", "Sweets", "#e84393"),
        ("other", "Other", "#6b7280"),
    ];
    for (id, label, color) in &defaults {
        conn.execute(
            "INSERT OR IGNORE INTO categories (id, label, color) VALUES (?1, ?2, ?3)",
            rusqlite::params![id, label, color],
        )
        .expect("Failed to insert default category");
    }
}
