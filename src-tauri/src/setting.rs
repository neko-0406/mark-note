use std::{
    fs::{self, File},
    io::BufReader,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::app_state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSetting {
    sidemenu_width: u64,
    work_directory: Option<String>,
}

impl AppSetting {
    pub fn load(app_handle: &AppHandle) -> Result<Mutex<AppSetting>, String> {
        // アプリのデータディレクトリの確認・作成
        let app_data_dir = app_handle.path().app_data_dir().map_err(|error| {
            format!(
                "アプリのデータディレクトリの取得に失敗しました: {}",
                error.to_string()
            )
        })?;
        if !app_data_dir.exists() {
            fs::create_dir(&app_data_dir).map_err(|error| {
                format!(
                    "アプリデータディレクトリの作成に失敗しました: {}",
                    error.to_string()
                )
            })?;
        }

        // コンフィグファイルの確認・作成
        let config_file = app_data_dir.join("app_setting.json");
        if !config_file.exists() {
            File::create(&config_file).map_err(|error| {
                format!(
                    "コンフィグファイルの作成に失敗しました: {}",
                    error.to_string()
                )
            })?;
        }

        // コンフィグファイルのロード
        let file = File::open(&config_file).map_err(|error| {
            format!(
                "コンフィグファイルのロードに失敗しました: {}",
                error.to_string()
            )
        })?;
        let reader = BufReader::new(file);

        let app_setting: AppSetting = match serde_json::from_reader(reader) {
            Ok(setting) => setting,
            Err(error) => {
                println!("コンフィグのロードに失敗しました: {}", error.to_string());
                AppSetting::default()
            }
        };

        Ok(Mutex::new(app_setting))
    }
}

impl Default for AppSetting {
    fn default() -> Self {
        Self {
            sidemenu_width: 50,
            work_directory: None,
        }
    }
}

#[tauri::command]
pub fn get_app_setting(state: tauri::State<'_, AppState>) -> Result<AppSetting, String> {
    let setting = state
        .app_setting
        .lock()
        .map_err(|error| format!("AppSettingの取得に失敗しました: {}", error.to_string()))?;
    Ok(setting.clone())
}

#[tauri::command]
pub fn update_app_setting(
    state: tauri::State<'_, AppState>,
    new_app_setting: AppSetting,
) -> Result<(), String> {
    let mut setting = state
        .app_setting
        .lock()
        .map_err(|error| format!("AppSettingの更新に失敗しました: {}", error.to_string()))?;
    *setting = new_app_setting;
    Ok(())
}
