use std::{
    fs::File,
    io::{BufReader, BufWriter},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SideMenubar {
    pub width: i64,
    pub display_ui: bool,
}

impl Default for SideMenubar {
    fn default() -> Self {
        Self {
            width: 0,
            display_ui: false,
        }
    }
}

impl SideMenubar {
    pub fn load(app_handle: &AppHandle) -> Result<Mutex<SideMenubar>, String> {
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|error| format!("データディレクトリの取得に失敗しました: {}", error.to_string()))?;
        let state_json_path = app_data_dir.join("app_state.json");

        if !state_json_path.exists() {
            let file = File::create(&state_json_path)
                .map_err(|error| format!("状態保存用ファイルの作成に失敗しました: {}", error.to_string()))?;
            let writer = BufWriter::new(file);
            let sidebar_state = SideMenubar::default();
            serde_json::to_writer_pretty(writer, &sidebar_state)
                .map_err(|error| format!("状態保存用ファイルの初期化に失敗しました: {}", error.to_string()))?;
        }

        let file = File::open(&state_json_path)
            .map_err(|error| format!("状態保存用ファイルのオープンに失敗しました: {}", error.to_string()))?;
        let reader = BufReader::new(file);
        let data: SideMenubar = serde_json::from_reader(reader)
            .map_err(|error| format!("サイドバーの状態の読み込みに失敗しました: {}", error))?;
        Ok(Mutex::new(data))
    }
}
