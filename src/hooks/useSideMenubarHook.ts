import { SideMenubarState } from '@/interface/sideMenuBar';
import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useRef, useState } from 'react';

export function useSideMenubarHook() {
  // サイドバーの状態を保存するためのステート
  const [state, setState] = useState<SideMenubarState | null>(null);
  // tauri::Stateへの更新関数を呼び出す間隔を記録(ms)
  const timerRef = useRef(1000);

  const handleMouseDown = useCallback(() => {}, []);

  // ステートをtauri::Stateから取得・初期化する処理
  useEffect(() => {
    const getSidebarState = async () => {
      try {
        const state: SideMenubarState = await invoke('get_sidemenubar_state');
        setState(state);
      } catch (error) {
        alert(`サイドバーの状態取得に失敗しました: ${error}`);
        setState(null);
      }
    };
    getSidebarState();
  }, []);
  // stateに変更があった場合新しいステートでtauri::Stateを更新
  useEffect(() => {
    // state更新用の関数
    const updateSidebarState = async () => {
      try {
        await invoke('update_sidemenubar_state', { newSidebarState: state });
      } catch (error) {
        alert(`サイドバーの状態更新に失敗しました: ${error}`);
      }
    };
    // タイマーが起動したらstateを更新
    const intervalId = setInterval(() => updateSidebarState(), timerRef.current);

    return () => {
      // 時間内に変更があればタイマーを破棄
      clearInterval(intervalId);
    };
  }, [state]);

  return { state };
}
