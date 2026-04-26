use tauri::Manager;

use crate::{
    app_state::AppState,
    setting::{get_app_setting, save_app_setting, update_app_setting, AppSetting},
    state::side_menubar::SideMenubar,
    tauri_commands::{get_sidemenubar_state, update_sidemenubar_state},
};

mod app_state;
mod setting;
mod state;
mod tauri_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| -> Result<(), Box<dyn std::error::Error>> {
            let app_setting = AppSetting::load(&app.app_handle())?;

            let side_menubar = SideMenubar::load(&app.app_handle())
                .map_err(|e| format!("サイドバーの状態の読み込みに失敗しました: {}", e.to_string()))?;

            app.manage(AppState {
                app_setting,
                side_menubar,
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_app_setting,
            update_app_setting,
            save_app_setting,
            get_sidemenubar_state,
            update_sidemenubar_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
