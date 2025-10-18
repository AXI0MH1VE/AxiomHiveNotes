# AxiomHive Notes

A cutting-edge, lightweight desktop notes application built with Tauri v2, React, and TypeScript. Part of the **@AxiomHive Collection**.

## 🚀 Features

- **Lightning Fast**: Tauri v2 native desktop app with <200ms startup and <120MB RAM footprint
- **Rich Editor**: TipTap-powered markdown editor with code blocks, tables, wiki-links, and slash commands
- **Instant Search**: SQLite FTS5 full-text search with BM25 ranking and snippet highlights (<20ms on 5K notes)
- **CRDT-Ready**: Yjs integration for future conflict-free sync
- **Optional Encryption**: App-level encryption with Argon2id + libsodium (secretbox)
- **Smart Organization**: Tags, folders, backlinks, and command palette (Ctrl/Cmd+K)
- **Reminders**: Native OS notifications via Tauri
- **Dark/Light Themes**: Axiom Hive branded color system
- **Auto-Updates**: Built-in updater via GitHub Releases
- **No Telemetry**: Privacy-first, local-only by default

## 🏗️ Architecture

| Layer | Technology | Purpose |
|-------|-----------|---------|
| Desktop Shell | Tauri v2 (Rust) | Native Windows/macOS/Linux with NSIS installer |
| Frontend | React 18 + TypeScript + Vite | Modern UI with hot-reload |
| UI Components | Tailwind CSS + shadcn/ui | Accessible, themable design system |
| Editor | TipTap (ProseMirror) | Extensible rich text with Markdown I/O |
| State | Zustand + TanStack Query | Client state + async data |
| Database | SQLite (rusqlite) + FTS5 | Embedded DB with full-text search |
| Migrations | Refinery | Type-safe schema versioning |
| CRDT | Yjs (local) | Future multi-device sync |
| Encryption | Argon2id + libsodium | Optional at-rest encryption |
| Reminders | Tokio scheduler | Async task scheduling |

## 📦 Performance Budgets

- **Startup**: <200ms to first paint
- **Memory**: <120MB idle
- **Search**: <20ms round-trip on 5K notes
- **Keystroke latency**: <8ms in editor
- **Installer size**: <30MB

## 🛠️ Development

### Prerequisites

- **Rust** (MSVC toolchain): `winget install Rustlang.Rust.MSVC`
- **Node.js** (LTS): `winget install OpenJS.NodeJS.LTS`
- **pnpm**: `npm i -g pnpm`
- **NSIS**: `winget install NSIS.NSIS`
- **Visual Studio Build Tools** (C++ workload)

### Quick Start

```bash
# Clone the repo
git clone https://github.com/{{YOUR_USERNAME}}/AxiomHiveNotes.git
cd AxiomHiveNotes

# Install frontend dependencies
cd app && pnpm install && cd ..

# Run in dev mode
pnpm tauri dev
```

### Build Installer

```bash
# Build frontend
pnpm -C app build

# Build Tauri (creates NSIS installer)
pnpm tauri build
```

Installer will be in `src-tauri/target/release/bundle/nsis/`

## 🗂️ Project Structure

```
AxiomHiveNotes/
├── app/                    # React frontend
│   ├── src/
│   │   ├── components/    # UI components
│   │   ├── lib/          # Utilities
│   │   └── App.tsx       # Main entry
│   ├── tailwind.config.js
│   └── package.json
├── src-tauri/             # Rust backend
│   ├── src/
│   │   └── lib.rs        # Tauri commands
│   ├── migrations/       # SQL migrations
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri config
└── README.md
```

## 🔒 Security

- **Encryption**: Optional, user-controlled with Argon2id key derivation
- **No telemetry by default**: Opt-in only
- **Local-first**: All data stored locally; no cloud dependency
- **Auto-updates**: Signed releases via GitHub (updater can be disabled)

## 🎨 Branding

Part of the **Axiom Hive Collection** — NFT-grade design, pixel-perfect execution, and @AxiomHive / @DevdollzAi integration at the data schema level.

## 📝 License

MIT

## 🙏 Acknowledgments

Built with:
- [Tauri](https://tauri.app)
- [React](https://react.dev)
- [TipTap](https://tiptap.dev)
- [shadcn/ui](https://ui.shadcn.com)
- [SQLite](https://sqlite.org)

---

**@AxiomHive** — Legendary, unbeatable supremacy in desktop productivity.
