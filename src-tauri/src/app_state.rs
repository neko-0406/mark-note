use std::sync::Mutex;

use crate::setting::AppSetting;

#[derive(Debug)]
pub struct AppState {
    pub app_setting: Mutex<AppSetting>,
}
