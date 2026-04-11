use tauri::State;

use crate::{app_state::AppState, state::side_menubar::SideMenubar};

/*
 * サイドメニューバーの状態を取得するコマンド
 */
#[tauri::command]
pub fn get_sidemenubar_state(state: State<'_, AppState>) -> Result<SideMenubar, String> {
    let side_menubar_state = state
        .side_menubar
        .lock()
        .map_err(|error| format!("サイドバーの状態の取得に失敗しました: {}", error.to_string()))?;
    Ok(side_menubar_state.clone())
}
/*
 * サイドメニューバーの状態を更新するコマンド
 */
#[tauri::command]
pub fn set_sidemenubar_state(state: State<'_, AppState>, new_sidebar_state: SideMenubar) -> Result<(), String> {
    let mut side_menubar_state = state
        .side_menubar
        .lock()
        .map_err(|error| format!("サイドバーの状態の取得に失敗しました: {}", error.to_string()))?;

    *side_menubar_state = new_sidebar_state;
    Ok(())
}
