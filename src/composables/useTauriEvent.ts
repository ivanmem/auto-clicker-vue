import { listen, type EventCallback, type EventName, type UnlistenFn } from '@tauri-apps/api/event'
import { tryOnMounted, tryOnUnmounted } from '@vueuse/core'

// Тонкая обёртка над `listen` из @tauri-apps/api/event с автоматической отпиской
// и защитой от гонки (если компонент успел размонтироваться раньше, чем разрешился `listen`).
export function useTauriEvent<T>(event: EventName, callback: EventCallback<T>) {
  let unlisten: UnlistenFn | undefined
  let cancelled = false

  tryOnMounted(async () => {
    const fn = await listen<T>(event, callback)
    if (cancelled) {
      fn()
    } else {
      unlisten = fn
    }
  })

  tryOnUnmounted(() => {
    cancelled = true
    unlisten?.()
  })
}
