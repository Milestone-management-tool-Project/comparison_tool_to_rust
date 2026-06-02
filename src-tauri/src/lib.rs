use std::sync::Mutex;
pub mod wrapper;
pub mod file_operations;
pub mod timer;
pub mod structure;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .manage(Mutex::new(structure::Timer{
        flag: false,
        start_time: None,
        end_time: None,
        total: None
    }))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            wrapper::start_timer_cmd,
            wrapper::stop_timer_cmd
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
