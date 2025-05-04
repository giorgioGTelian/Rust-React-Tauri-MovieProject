mod db;
mod scanner;
mod api;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::spawn(async move {
                let db = db::init_db().await.expect("Failed to initialize database");
                handle.manage(db);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![api::get_movies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}