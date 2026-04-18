import { invoke } from '@tauri-apps/api/core'
import { tryOnMounted } from '@vueuse/core'
import { watch } from 'vue'
import type { HotkeyMode } from '@/constants/clicker'
import { useTauriEvent } from './useTauriEvent'

// Подписывается на глобальный хоткей (Windows-хук на Rust-стороне).
// `mode` — геттер режима хоткея; при его смене бекенду посылается актуальное значение.
export function useGlobalHotkey(mode: () => HotkeyMode, onTrigger: () => void) {
  tryOnMounted(async () => {
    await invoke('start_hotkey_monitor')
    await invoke('set_hotkey_mode', { mode: mode() })
  })

  watch(mode, (value) => {
    invoke('set_hotkey_mode', { mode: value })
  })

  useTauriEvent('hotkey-triggered', onTrigger)
}
