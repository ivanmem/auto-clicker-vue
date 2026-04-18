import { invoke } from '@tauri-apps/api/core'
import { useIntervalFn, useTimeoutFn } from '@vueuse/core'
import { ref, watch } from 'vue'
import type { MouseButton } from '@/constants/clicker'

interface UseAutoClickerOptions {
  isRunning: () => boolean
  intervalMs: () => number
  selectedButton: () => MouseButton
  autoPauseOnMouseMove: () => boolean
  pauseDurationSec: () => number
  onClick: () => void
}

// Сердце кликера: периодически зовёт Rust-команду `click_mouse` и умеет
// уходить на «остывание» при сигнале активности от пользователя.
export function useAutoClicker(options: UseAutoClickerOptions) {
  const isPaused = ref(false)

  const interval = useIntervalFn(
    async () => {
      if (isPaused.value) {
        return
      }

      try {
        await invoke('click_mouse', { button: options.selectedButton() })
        options.onClick()
      } catch (error) {
        console.error('Не удалось выполнить клик:', error)
      }
    },
    options.intervalMs,
    { immediate: false },
  )

  const cooldown = useTimeoutFn(
    () => {
      isPaused.value = false
      if (options.isRunning()) {
        interval.resume()
      }
    },
    () => options.pauseDurationSec() * 1000,
    { immediate: false },
  )

  watch(options.isRunning, (running) => {
    isPaused.value = false
    cooldown.stop()
    if (running) {
      interval.resume()
    } else {
      interval.pause()
    }
  })

  watch(options.autoPauseOnMouseMove, (enabled) => {
    if (enabled || !isPaused.value) {
      return
    }

    isPaused.value = false
    cooldown.stop()
    if (options.isRunning()) {
      interval.resume()
    }
  })

  // Сообщить кликеру, что пользователь «шевелится» — взвести/продлить паузу.
  function reportActivity() {
    if (!options.isRunning() || !options.autoPauseOnMouseMove()) {
      return
    }

    if (!isPaused.value) {
      isPaused.value = true
      interval.pause()
    }

    cooldown.start()
  }

  return { isPaused, reportActivity }
}
