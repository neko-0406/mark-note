use tauri::Manager;

use crate::{
    app_state::AppState,
    setting::{get_app_setting, AppSetting},
};

mod app_state;
mod setting;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let app_setting = AppSetting::load(&app.app_handle())?;

            app.manage(AppState { app_setting });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_app_setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
