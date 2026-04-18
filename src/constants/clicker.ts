// Кнопки/действия мыши, которые умеет эмулировать Rust-команда `click_mouse`.
export type MouseButton = 'Left' | 'Right' | 'Middle' | 'WheelDown' | 'Enter'

export interface ButtonOption {
  value: MouseButton
  label: string
}

export const BUTTON_OPTIONS: ButtonOption[] = [
  { value: 'Left', label: 'ЛКМ' },
  { value: 'Right', label: 'ПКМ' },
  { value: 'Middle', label: 'СКМ' },
  { value: 'WheelDown', label: 'Колёсико ↓' },
  { value: 'Enter', label: 'Enter' },
]

// Режимы глобальной горячей клавиши «старт/пауза».
// Числа должны строго совпадать со значениями `HOTKEY_MODE` в src-tauri/src/lib.rs.
export const HotkeyMode = {
  None: 0,
  F8: 1,
  MiddleClick: 2,
  LeftClick: 3,
  RightClick: 4,
} as const

export type HotkeyMode = (typeof HotkeyMode)[keyof typeof HotkeyMode]

export interface HotkeyOption {
  value: HotkeyMode
  label: string
}

export const HOTKEY_OPTIONS: HotkeyOption[] = [
  { value: HotkeyMode.None, label: 'Нет' },
  { value: HotkeyMode.F8, label: 'F8' },
  { value: HotkeyMode.MiddleClick, label: 'Колёсико (нажатие)' },
  { value: HotkeyMode.LeftClick, label: 'ЛКМ' },
  { value: HotkeyMode.RightClick, label: 'ПКМ' },
]
