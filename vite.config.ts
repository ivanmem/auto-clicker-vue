import ui from '@nuxt/ui/vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import { nodePolyfills } from 'vite-plugin-node-polyfills'
import topLevelAwait from 'vite-plugin-top-level-await'
import vueDevTools from 'vite-plugin-vue-devtools'
import { version as pkgVersion } from './package.json'

const HOST = process.env.TAURI_DEV_HOST
const PLATFORM = process.env.TAURI_ENV_PLATFORM
process.env.VITE_APP_VERSION = pkgVersion
if (process.env.NODE_ENV === 'production') {
  process.env.VITE_APP_BUILD_EPOCH = new Date().getTime().toString()
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    topLevelAwait(),
    nodePolyfills(),
    vue(),
    // Nuxt UI Vite plugin: подтягивает Tailwind CSS 4, темизацию,
    // а также сам регистрирует unplugin-auto-import и unplugin-vue-components.
    // router: false — отключаем интеграцию с vue-router (одностраничное desktop-приложение).
    ui({
      router: false,
      autoImport: {
        imports: [
          'vue',
          'pinia',
          {
            '@/store': ['useStore'],
          },
        ],
        dts: 'auto-imports.d.ts',
        vueTemplate: true,
      },
      components: {
        dts: 'components.d.ts',
      },
    }),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  css: {
    preprocessorMaxWorkers: true,
  },

  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],
  server: {
    port: 1420,
    strictPort: true,
    host: HOST || false,
    hmr: HOST
      ? {
          protocol: 'ws',
          host: HOST,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  build: {
    outDir: './dist',
    // Целевые версии — Vite 8 baseline-widely-available, см. https://web-platform-dx.github.io/web-features/
    // Для Tauri WebView подробности здесь: https://v2.tauri.app/reference/webview-versions/
    target: PLATFORM == 'windows' ? 'chrome111' : 'safari16.4',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    emptyOutDir: true,
    chunkSizeWarningLimit: 1024,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
})
