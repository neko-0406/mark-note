import { ReactNode } from 'react';
import { IconType } from 'react-icons';

export interface SideMenubarState {
  width: number;
  displayUI: boolean;
}

export interface SideMenubarElement {
  label: string;
  icon: IconType;
  component: ReactNode;
}
