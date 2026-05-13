<script setup lang="ts">
import { useEventListener } from '@vueuse/core'
import { debounce } from 'es-toolkit'
import { computed } from 'vue'
import { useAutoClicker } from '@/composables/useAutoClicker'
import { useGlobalHotkey } from '@/composables/useGlobalHotkey'
import { useGlobalMouseActivity } from '@/composables/useGlobalMouseActivity'
import { BUTTON_OPTIONS, HOTKEY_OPTIONS } from '@/constants/clicker'
const store = useStore()

const { isPaused, reportActivity } = useAutoClicker({
  isRunning: () => store.isRunning,
  intervalMs: () => store.intervalMs,
  selectedButton: () => store.selectedButton,
  autoPauseOnMouseMove: () => store.autoPauseOnMouseMove,
  pauseDurationSec: () => store.pauseDurationSec,
  onClick: store.incrementClickCount,
})

// Глобальное движение мыши приходит часто — дебаунсим, чтобы не дёргать таймер впустую.
const handleMouseActivity = debounce(reportActivity, 100)

useGlobalMouseActivity(handleMouseActivity)
useGlobalHotkey(() => store.hotkeyMode, store.toggleRunning)

// Если окно потеряло фокус — пользователь почти наверняка переключился, ставим паузу.
useEventListener(window, 'blur', reportActivity)

const statusBadge = computed(() => {
  if (!store.isRunning) {
    return { color: 'neutral', icon: 'i-lucide-circle-off', label: 'Остановлен' } as const
  }

  if (isPaused.value) {
    return { color: 'warning', icon: 'i-lucide-pause-circle', label: 'Пауза (движение мыши)' } as const
  }

  return { color: 'success', icon: 'i-lucide-circle-play', label: 'Работает' } as const
})
</script>

<template>
  <UCard
    class="w-full max-w-md shadow-lg"
    :ui="{ root: 'bg-elevated/30 backdrop-blur-sm', body: 'space-y-5' }"
  >
    <template #header>
      <div class="flex items-center justify-between gap-3">
        <UBadge
          :color="statusBadge.color"
          :icon="statusBadge.icon"
          variant="subtle"
          size="lg"
          :label="statusBadge.label"
        />
        <div class="text-sm text-muted tabular-nums" aria-label="Счётчик кликов">
          <span class="text-toned font-semibold text-base">{{ store.clickCount }}</span>
          <span class="ml-1">кликов</span>
        </div>
      </div>
    </template>

    <UButton
      block
      size="xl"
      :color="store.isRunning ? 'error' : 'primary'"
      :icon="store.isRunning ? 'i-lucide-pause' : 'i-lucide-play'"
      :label="store.isRunning ? 'Пауза' : 'Старт'"
      @click="store.toggleRunning()"
    />

    <UFormField label="Интервал" hint="миллисекунды">
      <UInputNumber
        v-model="store.intervalMs"
        :min="10"
        :max="60000"
        :step="10"
        :disabled="store.isRunning"
        class="w-full"
      />
    </UFormField>

    <UFormField label="Кнопка мыши / клавиша">
      <URadioGroup
        v-model="store.selectedButton"
        :items="BUTTON_OPTIONS"
        :disabled="store.isRunning"
        orientation="horizontal"
        variant="card"
        :ui="{ fieldset: 'flex flex-wrap gap-2' }"
      />
    </UFormField>

    <USeparator />

    <UFormField>
      <div class="flex items-center justify-between gap-3">
        <span class="text-sm">Пауза при движении мыши</span>
        <USwitch v-model="store.autoPauseOnMouseMove" />
      </div>
    </UFormField>

    <UFormField label="Длительность паузы" hint="секунды">
      <UInputNumber
        v-model="store.pauseDurationSec"
        :min="1"
        :max="60"
        :step="1"
        :disabled="!store.autoPauseOnMouseMove"
        class="w-full"
      />
    </UFormField>

    <USeparator />

    <UFormField label="Горячая клавиша" hint="старт / пауза">
      <URadioGroup
        v-model="store.hotkeyMode"
        :items="HOTKEY_OPTIONS"
        orientation="horizontal"
        variant="card"
        :ui="{ fieldset: 'flex flex-wrap gap-2' }"
      />
    </UFormField>
  </UCard>
</template>
