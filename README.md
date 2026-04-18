<div align="center">

# Auto Clicker Vue

**Лёгкий, быстрый и красивый автокликер для Windows.**
Сделан на Tauri 2 + Vue 3 + Nuxt UI 4.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=white)](https://tauri.app)
[![Vue](https://img.shields.io/badge/Vue-3.5-42b883?logo=vue.js&logoColor=white)](https://vuejs.org)
[![Nuxt UI](https://img.shields.io/badge/Nuxt%20UI-4-00DC82?logo=nuxt&logoColor=white)](https://ui.nuxt.com)
[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Build](https://github.com/ivanmem/auto-clicker-vue/actions/workflows/test-build.yml/badge.svg)](https://github.com/ivanmem/auto-clicker-vue/actions/workflows/test-build.yml)
[![Tests](https://github.com/ivanmem/auto-clicker-vue/actions/workflows/test.yml/badge.svg)](https://github.com/ivanmem/auto-clicker-vue/actions/workflows/test.yml)

</div>

---

## Содержание

- [О проекте](#о-проекте)
- [Возможности](#возможности)
- [Скриншоты](#скриншоты)
- [Технологии](#технологии)
- [Установка](#установка)
- [Сборка из исходников](#сборка-из-исходников)
- [Структура проекта](#структура-проекта)
- [Как это работает](#как-это-работает)
- [Разработка](#разработка)
- [Тестирование](#тестирование)
- [Релиз новой версии](#релиз-новой-версии)
- [Дорожная карта](#дорожная-карта)
- [Вклад в проект](#вклад-в-проект)
- [Лицензия](#лицензия)

## О проекте

**Auto Clicker Vue** — настольное приложение для автоматического выполнения кликов мышью или нажатий клавиш с заданным интервалом. В отличие от тяжеловесных Electron-аналогов, программа собирается в нативный бинарник Windows весом в несколько мегабайт благодаря Tauri 2 и почти не потребляет оперативной памяти.

> Окно занимает буквально пару мегабайт памяти, а кликер обрабатывает события мыши и клавиатуры через низкоуровневые WinAPI-хуки прямо из Rust — без задержек и без скриптовых движков в фоне.

## Возможности

- **Гибкий интервал** — от 10 мс до минуты, шаг 10 мс.
- **5 типов действий** — левая, правая и средняя кнопки мыши, прокрутка колеса вниз, нажатие `Enter`.
- **Глобальные горячие клавиши** для старт/пауза:
  - `F8`
  - средняя кнопка мыши (нажатие колеса)
  - левая кнопка мыши
  - правая кнопка мыши
- **Авто-пауза при движении мыши** с настраиваемым «остыванием» (1–60 секунд) — удобно, когда работаешь параллельно за компьютером.
- **Счётчик кликов** в текущей сессии в реальном времени.
- **Сохранение всех настроек** между запусками (через `localStorage`).
- **Тёмная тема** Nuxt UI 3 «из коробки».
- **Маленький размер** установщика и потребление памяти меньше 50 МБ.

## Скриншоты

Скриншоты появятся здесь после первого релиза. Если запускали локально — буду рад PR со свежими снимками.

## Технологии

| Слой              | Стек                                                                                       |
| ----------------- | ------------------------------------------------------------------------------------------ |
| Бекенд (нативный) | [Tauri 2](https://tauri.app), [Rust](https://www.rust-lang.org), [enigo](https://crates.io/crates/enigo), `windows` crate, низкоуровневые WinAPI-хуки (`SetWindowsHookExW`) |
| Фронтенд          | [Vue 3.5](https://vuejs.org), [Pinia 3](https://pinia.vuejs.org), [VueUse 12](https://vueuse.org), [es-toolkit](https://es-toolkit.dev) |
| UI                | [Nuxt UI 4](https://ui.nuxt.com) (standalone Vue plugin) поверх [Reka UI](https://reka-ui.com) и [Tailwind CSS 4](https://tailwindcss.com) |
| Иконки            | [Lucide](https://lucide.dev) через [Iconify](https://iconify.design)                       |
| Сборка / DX       | [Vite 8](https://vite.dev), [TypeScript 6](https://www.typescriptlang.org), `unplugin-auto-import`, `unplugin-vue-components` |
| Тесты             | [Vitest 4](https://vitest.dev), `@vue/test-utils`, `@pinia/testing`                        |
| CI/CD             | GitHub Actions: тесты, сборка, релизы под Windows / Linux / macOS                          |

## Установка

> ⚠️ Поддерживается только **Windows** — глобальные горячие клавиши и слежение за мышью реализованы через WinAPI. На macOS / Linux UI запустится, но низкоуровневые хуки работать не будут.

1. Перейдите в раздел [Releases](https://github.com/ivanmem/auto-clicker-vue/releases).
2. Скачайте `.msi` или `.exe` для Windows.
3. Установите и запустите. Антивирус может «ругаться» на программу — это типичная история для автокликеров; код полностью открыт, можно собрать самому.

## Сборка из исходников

### Требования

- **Node.js** ≥ 22 и [pnpm](https://pnpm.io) ≥ 10.
- **Rust** stable (через [rustup](https://rustup.rs)).
- Зависимости Tauri 2 для вашей ОС — см. [официальный гайд](https://tauri.app/start/prerequisites/).

### Запуск в режиме разработки

```sh
pnpm install
pnpm tauri:dev
```

Откроются нативное окно приложения и DevTools (в debug-сборке). Hot reload работает и для Vue, и для Rust.

### Production-сборка

```sh
pnpm tauri:build
```

Готовые установщики окажутся в `src-tauri/target/release/bundle/`.

## Структура проекта

```
.
├── src/                          # Vue 3 фронтенд
│   ├── assets/main.css           # Tailwind 4 + Nuxt UI стили
│   ├── components/
│   │   └── ClickerControl.vue    # Чистый UI поверх композаблов
│   ├── composables/
│   │   ├── useAutoClicker.ts     # Цикл клика и логика «остывания»
│   │   ├── useGlobalHotkey.ts    # Подписка на глобальный хоткей
│   │   ├── useGlobalMouseActivity.ts # Глобальные движения мыши
│   │   └── useTauriEvent.ts      # Безопасная обёртка над `listen`
│   ├── constants/
│   │   └── clicker.ts            # MouseButton, HotkeyMode, опции
│   ├── App.vue                   # Корневой компонент с <UApp>
│   ├── main.ts                   # Регистрация Pinia и Nuxt UI Vue plugin
│   └── store.ts                  # Тонкий Pinia-стор настроек
├── src-tauri/                    # Rust бекенд
│   ├── src/
│   │   ├── lib.rs                # Команды Tauri и WinAPI-хуки
│   │   └── main.rs               # Точка входа
│   ├── capabilities/             # Декларации разрешений Tauri
│   ├── icons/                    # Иконки приложения
│   └── tauri.conf.json           # Конфиг Tauri 2
├── tests/                        # Vitest тесты и setup
├── .github/workflows/            # CI: test.yml, test-build.yml, release.yml
├── vite.config.ts                # Vite + Nuxt UI Vite plugin
└── README.md
```

## Как это работает

### Архитектура фронтенда

Логика разнесена по композаблам, чтобы `ClickerControl.vue` оставался чистым UI:

- **`useAutoClicker`** — крутит `useIntervalFn`, при необходимости уходит на «остывание» через `useTimeoutFn` и реагирует на смену настроек через `watch`. Принимает геттеры всех нужных полей, ничего не знает про Pinia.
- **`useGlobalMouseActivity`** — на старте зовёт Rust-команду `start_mouse_monitor` и пробрасывает каждое движение в callback.
- **`useGlobalHotkey`** — то же для горячей клавиши плюс синхронизирует выбранный режим с бекендом.
- **`useTauriEvent`** — тонкая обёртка над `listen` из `@tauri-apps/api/event` с автоматической отпиской и защитой от гонки «компонент успел размонтироваться раньше промиса».

Стор (`src/store.ts`) хранит только пользовательские настройки и runtime-состояние (`isRunning`, `clickCount`), всё через `useLocalStorage` из VueUse под префиксом `clicker-settings:*`.

### Глобальные хуки на Rust

Слежение за мышью и горячие клавиши реализованы через WinAPI-хуки `WH_MOUSE_LL` и `WH_KEYBOARD_LL`. Они ставятся в отдельных потоках при старте приложения и эмитят события `mouse-moved` / `hotkey-triggered` в WebView через `tauri::AppHandle::emit`.

DOM-события `mousemove` и `keydown` срабатывают только когда окно приложения в фокусе — а нам нужны глобальные подписки.

### Симуляция кликов

Для самих кликов используется кроссплатформенная библиотека [`enigo`](https://crates.io/crates/enigo). Это позволяет относительно легко расширить поддержку на macOS и Linux в будущем — в коде уже есть `Enter` через `Keyboard::key`.

### Авто-пауза при движении мыши

Каждое событие движения дебаунсится через `es-toolkit`'овский `debounce(100)`, после чего вызывается `reportActivity` из `useAutoClicker` — он останавливает интервал и взводит таймер «остывания». При следующем движении таймаут перезапускается, кликер возобновится только спустя N секунд тишины. Дополнительно ставим паузу при потере фокуса окна (`window.blur`).

## Разработка

| Скрипт                | Что делает                                                |
| --------------------- | --------------------------------------------------------- |
| `pnpm dev`            | Только Vite dev-сервер (без Tauri).                       |
| `pnpm tauri:dev`      | Полноценный dev-режим с нативным окном и hot reload.      |
| `pnpm build`          | Type-check + сборка фронтенда в `dist/`.                  |
| `pnpm tauri:build`    | Сборка нативных установщиков под текущую ОС.              |
| `pnpm test`           | Запуск Vitest-тестов.                                     |
| `pnpm type-check`     | Только проверка типов через `vue-tsc`.                    |
| `pnpm check`          | `cargo check` для бекенда.                                |
| `pnpm bump x.y.z`     | Атомарно поднимает версию в `package.json`, `tauri.conf.json` и `Cargo.toml`. |

### Рекомендуемые расширения VS Code

В `.vscode/extensions.json` уже указаны нужные расширения (Volar, Tauri, rust-analyzer и т.д.) — VS Code предложит их установить при открытии проекта. Для отладки Rust-части доступна конфигурация `Debug Tauri` в `.vscode/launch.json`.

## Тестирование

Юнит-тесты для фронтенда лежат в `tests/unit/` и запускаются через Vitest:

```sh
pnpm test
```

В `tests/setup/` уже подключены `@pinia/testing` и глобальные моки.

## Релиз новой версии

1. Поднимаем версию: `pnpm bump 1.2.3`.
2. Обновляем `Cargo.lock`: `pnpm check`.
3. Коммитим и тегаем: `git tag v1.2.3 && git push --tags`.
4. Workflow [`release.yml`](./.github/workflows/release.yml) автоматически соберёт релиз для Windows / Linux / macOS и создаст черновик GitHub Release.
5. Редактируем заметки и публикуем 🎉

## Дорожная карта

- [ ] Поддержка macOS и Linux для глобальных хуков.
- [ ] Серии кликов (последовательности) и сценарии.
- [ ] Случайный джиттер интервала, чтобы лучше имитировать живого пользователя.
- [ ] Системный трей и сворачивание в фон.
- [ ] Светлая тема (toggle).
- [ ] Локализация (en/ru).

## Вклад в проект

PR и issue приветствуются. Пожалуйста, [ведите себя адекватно](./CODE_OF_CONDUCT.md) при общении и старайтесь придерживаться существующего стиля кода (Vue 3 `<script setup>`, declaration-style функции внизу, никаких ненужных функциональных оберток).

## Лицензия

[MIT](./LICENSE) — делайте с кодом что угодно, никаких гарантий не предоставляется.
