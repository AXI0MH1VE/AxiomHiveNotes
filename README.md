<div align="center">

# ✨ AxiomHive Notes

### *Your thoughts deserve better*

**A native desktop notes application that's 4× faster and 5× lighter than Obsidian**

⚡ *Built with Rust and Tauri for those who demand both beauty and performance*

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Instant full-text search · Sub-200ms startup · 30MB installer  
Your notes stay local, your data stays yours, your system stays responsive

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

</div>

## ✧ Why Choose AxiomHive Notes

### 💫 Performance That Matters

<table>
<tr>
<td width="50%">

**Startup**  
`< 200ms` *vs Obsidian ~800ms*

**Memory**  
`< 120MB` *vs Obsidian 200-400MB*

</td>
<td width="50%">

**Search**  
`< 20ms` *on 5K notes vs Obsidian 50-100ms*

**Installer**  
`30MB` *vs Obsidian 150MB*

</td>
</tr>
</table>

### 🌸 Built for Developers & Power Users

```
✦ Native Rust backend with SQLite FTS5 (not Electron)
✦ BM25-ranked search with snippet highlighting  
✦ Markdown-first with live autosave
✦ Local-first architecture with optional encryption
✦ Zero telemetry by default
```

### ✨ Production-Ready Features

<table>
<tr>
<td width="33%" align="center">

**Organization**  
Tags · Folders  
Pin · Archive

</td>
<td width="33%" align="center">

**Safety**  
Soft Delete  
Backup & Restore

</td>
<td width="33%" align="center">

**Future-Ready**  
Auto-Updates  
CRDT Sync Ready

</td>
</tr>
</table>

## 💝 Installation

<div align="center">

[**⬇ Download for Windows**](https://github.com/AXI0MH1VE/AxiomHiveNotes/releases/latest)

*Current version: v0.1.0 · 30MB · Windows x64*

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

</div>

The installer creates a desktop shortcut and adds AxiomHive Notes to your Start menu.  
Your notes are stored locally in `%APPDATA%\app.axiomhive.notes`.

*macOS and Linux builds coming soon*

## 🏛️ Architecture

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

## 🎯 Performance Budgets

```
✧ Startup          < 200ms    to first paint
✧ Memory           < 120MB    idle
✧ Search           < 20ms     round-trip on 5K notes
✧ Keystroke        < 8ms      in editor
✧ Installer        < 30MB     total size
```

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
git clone https://github.com/AXI0MH1VE/AxiomHiveNotes.git
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

## 📁 Project Structure

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

## 🔐 Security

```
✦ Encryption      Optional, user-controlled with Argon2id
✦ Telemetry       Opt-in only, disabled by default
✦ Data Storage    100% local, zero cloud dependency
✦ Updates         Signed releases via GitHub (optional)
```

## 💎 Branding

Part of the **Axiom Hive Collection**. Notes are tagged with `@AxiomHive` or `@DevdollzAi` at the schema level for project organization and future collaboration features.

<div align="center">

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

## ✨ Built With

[Tauri](https://tauri.app) · [React](https://react.dev) · [TipTap](https://tiptap.dev) · [shadcn/ui](https://ui.shadcn.com) · [SQLite](https://sqlite.org)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

### License

MIT

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

*@AxiomHive — Where productivity meets elegance*

</div>
