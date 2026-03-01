use rusqlite::Connection;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::Manager;

/// Wrapper around a SQLite connection so it can be managed as Tauri state.
/// The Mutex ensures that concurrent command invocations do not race on the
/// single connection.
pub struct DbState {
    pub conn: Mutex<Connection>,
    pub db_path: String,
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
        db_path: ":memory:".to_string(),
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
        db_path: db_path.to_string_lossy().into_owned(),
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
        ("boisson-sans-alcool", "Boisson sans alcool", "#3b82f6"),
        ("alcool", "Alcool", "#8b5cf6"),
        ("sucreries", "Sucreries", "#e84393"),
        ("autre", "Autre", "#6b7280"),
    ];
    for (id, label, color) in &defaults {
        conn.execute(
            "INSERT OR IGNORE INTO categories (id, label, color) VALUES (?1, ?2, ?3)",
            rusqlite::params![id, label, color],
        )
        .expect("Failed to insert default category");
    }

    let default_products: [(&str, &str, i64, &str); 20] = [
        ("the", "Thé", 100, "boisson-sans-alcool"),
        ("cafe", "Café", 100, "boisson-sans-alcool"),
        ("soda", "Soda", 200, "boisson-sans-alcool"),
        ("jus-de-fruit", "Jus de fruit", 200, "boisson-sans-alcool"),
        ("biere-pichet", "Bière (pichet)", 1200, "alcool"),
        ("biere-25cl", "Bière (25cl)", 300, "alcool"),
        ("cidre-doux", "Cidre (doux)", 300, "alcool"),
        ("cidre-brut", "Cidre (brut)", 300, "alcool"),
        ("consigne-verre", "Consigne verre", 100, "autre"),
        ("consigne-pichet", "Consigne pichet", 500, "autre"),
        ("bonbon", "Bonbon/M&Ms/Twix", 100, "sucreries"),
        ("part-de-gateau", "Part de gâteau", 100, "sucreries"),
        ("crepe-nature", "Crêpe nature", 200, "sucreries"),
        ("crepe-sucre", "Crêpe au sucre", 250, "sucreries"),
        ("crepe-confiture", "Crêpe à la confiture", 350, "sucreries"),
        ("crepe-caramel", "Crêpe au caramel", 350, "sucreries"),
        ("crepe-nutella", "Crêpe au Nutella", 350, "sucreries"),
        ("cake-sale", "Cake salé", 100, "snack"),
        ("sandwich", "Sandwich", 400, "snack"),
        ("panini", "Panini", 400, "snack"),
    ];
    for (id, name, price, category_id) in &default_products {
        conn.execute(
            "INSERT OR IGNORE INTO products (id, name, price, category_id) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![id, name, price, category_id],
        )
        .expect("Failed to insert default product");
    }
}
