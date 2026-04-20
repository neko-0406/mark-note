import { useSideMenubarHook } from '@/hooks/useSideMenubarHook';
import style from '@/styles/Sidemenubar.module.css';

export default function Sidemenubar(): React.ReactNode {
  const { state } = useSideMenubarHook();

  return (
    <div className={style.Container}>
      <div className={style.IconArea}></div>
      <div className={style.PlotArea} style={{ width: `${state ? state.width : 0}px` }}></div>
      <div className={style.Splitter}></div>
    </div>
  );
}
