import { useLocalStorage } from '@vueuse/core'
import { acceptHMRUpdate, defineStore } from 'pinia'
import { ref } from 'vue'
import { HotkeyMode, type MouseButton } from '@/constants/clicker'

const STORAGE_KEY = 'clicker-settings'

// Главный стор приложения хранит только пользовательские настройки и runtime-состояние.
// Низкоуровневая логика клика и подписок живёт в композаблах (см. src/composables/).
export const useStore = defineStore('main', () => {
  const isRunning = ref(false)
  const clickCount = ref(0)

  const selectedButton = useLocalStorage<MouseButton>(`${STORAGE_KEY}:selectedButton`, 'Left')
  const intervalMs = useLocalStorage<number>(`${STORAGE_KEY}:intervalMs`, 1000)
  const autoPauseOnMouseMove = useLocalStorage<boolean>(`${STORAGE_KEY}:autoPauseOnMouseMove`, true)
  const pauseDurationSec = useLocalStorage<number>(`${STORAGE_KEY}:pauseDurationSec`, 5)
  // useLocalStorage сам понимает примитивные числа — кастомный сериалайзер не нужен.
  const hotkeyMode = useLocalStorage<HotkeyMode>(`${STORAGE_KEY}:hotkeyMode`, HotkeyMode.F8)

  function toggleRunning() {
    isRunning.value = !isRunning.value
    if (isRunning.value) {
      clickCount.value = 0
    }
  }

  function incrementClickCount() {
    clickCount.value++
  }

  return {
    isRunning,
    clickCount,
    selectedButton,
    intervalMs,
    autoPauseOnMouseMove,
    pauseDurationSec,
    hotkeyMode,
    toggleRunning,
    incrementClickCount,
  }
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useStore, import.meta.hot))
}
