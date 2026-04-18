import { invoke } from '@tauri-apps/api/core'
import { tryOnMounted } from '@vueuse/core'
import { useTauriEvent } from './useTauriEvent'

// Запускает на бекенде низкоуровневое наблюдение за позицией курсора и
// прокидывает каждое заметное движение в `onMove` (вне зависимости от того,
// активно ли окно нашего приложения).
export function useGlobalMouseActivity(onMove: () => void) {
  tryOnMounted(() => invoke('start_mouse_monitor'))
  useTauriEvent('mouse-moved', onMove)
}
