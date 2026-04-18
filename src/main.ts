import ui from '@nuxt/ui/vue-plugin'
import { LogicalSize } from '@tauri-apps/api/dpi'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import App from './App.vue'
import './assets/main.css'

const app = createApp(App)

app.use(createPinia())
app.use(ui)

app.mount('#app')

// Подгоняем высоту окна под реальный контент после рендера.
// Работает корректно при любом масштабировании Windows.
requestAnimationFrame(async () => {
  const contentHeight = document.body.scrollHeight
  if (contentHeight > 0) {
    const win = getCurrentWindow()
    const currentSize = await win.innerSize()
    const scaleFactor = await win.scaleFactor()
    const currentLogicalWidth = currentSize.width / scaleFactor
    await win.setSize(new LogicalSize(currentLogicalWidth, contentHeight))
  }
})
