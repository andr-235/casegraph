# casegraph

Tauri v2 + React 19 + TypeScript + SQLite desktop app for case management.

## Quick start

```bash
npm install
npm run tauri dev     # launches Vite + Tauri dev window
```

## Key commands

| Command | What it does |
|---|---|
| `npm run dev` | Vite dev server at `127.0.0.1:1420` (strict port) |
| `npm run build` | `tsc && vite build` (no separate lint step) |
| `npm run typecheck` | `tsc --noEmit` |
| `npm run tauri` | Tauri CLI passthrough (e.g. `npm run tauri build`) |

No linter or formatter config exists in the repo.

## Architecture

**Tauri commands** — all registered in `src-tauri/src/lib.rs:21-34`. Every command:
- Accepts a single `payload` struct argument (`rename_all = "camelCase"`)
- Returns `CommandResult<T>` — a custom Result enum `{ ok: true, data: T } | { ok: false, error: { code, message, details? } }`

**Frontend** — `src/main.tsx` → `App.tsx` (state machine, no router library). Routing is driven by a `BootstrapState` enum:
`loading → firstAdminRequired → loginRequired → authenticated`

Feature directories under `src/features/{auth,cases,materials}/` each contain `api/` (Tauri invocations) and `model/` (TypeScript types).

**Backend layers** — `commands/` → `services/` → `repositories/` → `db/` (rusqlite, SQLite). DB path: `app_data_dir / "casegraph.sqlite"`. Migrations auto-run in `initialize_app`.

**Auth** — argon2 password hashing, in-memory session via `SessionState`, 3 seeded roles: `administrator`, `analyst`, `viewer`.

## Conventions

- API field naming: **camelCase** everywhere (Rust serde `rename_all = "camelCase"`, TypeScript mirrors it)
- UI strings are in **Russian**
- Types in `features/{name}/model/` mirror Rust DTOs exactly
- `src/shared/api/invoke.ts` wraps `@tauri-apps/api/core::invoke` and unwraps `CommandResult`, throwing `AppCommandError` on error
- Offline-only app (`offlineMode: true` always)
- No tests exist in the repo yet
