use std::sync::Mutex;

use crate::{setting::AppSetting, state::side_menubar::SideMenubar};

#[derive(Debug)]
pub struct AppState {
    pub app_setting: Mutex<AppSetting>,
    pub side_menubar: Mutex<SideMenubar>,
}
