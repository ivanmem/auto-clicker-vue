<script setup lang="ts">
import { getCurrentWindow, PhysicalSize } from '@tauri-apps/api/window'
import { useDevicePixelRatio, useResizeObserver } from '@vueuse/core'
import { debounce } from 'es-toolkit'
import { ref } from 'vue'
import ClickerControl from './components/ClickerControl.vue'

const mainRef = ref<HTMLElement | null>(null)
const { pixelRatio } = useDevicePixelRatio()

let lastW = 0
let lastH = 0
let titlebarH = 0 // физические пиксели

// Вычисляем высоту titlebar один раз (outerSize - innerSize)
getCurrentWindow().outerSize().then(async outer => {
  const inner = await getCurrentWindow().innerSize()
  titlebarH = outer.height - inner.height
})

const syncSize = debounce(() => {
  const el = mainRef.value
  if (!el) return
  const dpr = pixelRatio.value || 1
  const width = Math.ceil(el.scrollWidth * dpr)
  const height = Math.ceil(el.scrollHeight * dpr) + titlebarH
  if (width > 0 && height > 0 && (width !== lastW || height !== lastH)) {
    lastW = width
    lastH = height
    getCurrentWindow().setSize(new PhysicalSize(width, height)).catch(() => {})
  }
}, 50)

useResizeObserver(mainRef, syncSize)
</script>

<template>
  <UApp>
    <main ref="mainRef" class="flex items-center justify-center bg-default text-default self-start w-full">
      <ClickerControl />
    </main>
  </UApp>
</template>
