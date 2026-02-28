mod commands;
mod db;
mod models;

use commands::*;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db_state = db::init_db(app.handle())
                .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            app.manage(db_state);

            #[cfg(debug_assertions)]
            if let Some(window) = app.get_webview_window("main") {
                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_categories,
            create_category,
            update_category,
            list_products,
            create_product,
            update_product,
            toggle_product_availability,
            create_order,
            list_orders,
            get_dashboard_summary,
            reset_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
