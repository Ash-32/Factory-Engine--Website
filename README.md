# EngineVault

Native Windows desktop software that reads the NTFS Master File Table directly, classifies engineering files, and displays a **branch-tree dashboard** — built for manufacturing / quality / CAD teams.

## Quick start (install & run)

### Prerequisites

1. [Rust stable](https://rustup.rs/)
2. [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) — **Desktop development with C++**
3. Windows 10/11 NTFS volume

### Build & install

```powershell
cd path\to\ntfs-catalog
.\install\build.ps1
.\install\install.ps1
```

This installs **`EngineVault.exe`** to `C:\Program Files\EngineVault\` and adds a Start Menu shortcut.

### First run

1. Launch **EngineVault** from the Start Menu.
2. The **demo catalog** loads automatically — explore the branch tree immediately.
3. For a **live scan**: right-click → **Run as administrator** → click **Scan Drive**.

## Dashboard

| Panel | Purpose |
|-------|---------|
| **Overview** | File counts, sizes, part branches, unclassified bucket |
| **Branch Explorer** | Category → part/revision branches → files (tree) |
| **File detail** | Path, confidence, timestamps, re-label corrections |

Branch structure example:

```
📁 Drawing (4)
  🌿 ABC-100 · Rev A  ·  2 files
     📎 ABC-100_REV-A.pdf
     📎 ABC-100_REV-A.dwg
  🌿 ABC-100 · Rev B  ·  1 file
📁 Quality (3)
  🌿 Widget-FMEA · Rev …
```

## Architecture (local-first → SaaS-ready)

```
┌─────────────────────────────────────────────────────────┐
│  EngineVault.exe  (eframe GUI — no browser, no Node.js) │
├─────────────────────────────────────────────────────────┤
│  Dashboard layer   branch tree · stats · corrections    │
│  Classify layer    TOML rules · confidence · grouping   │
│  Catalog layer     MFT read · mmap cache · USN watch    │
├─────────────────────────────────────────────────────────┤
│  Security boundary (src/security/)                      │
│  · All data under %LOCALAPPDATA%\EngineVault\           │
│  · Zero outbound network in core binary                 │
│  · Append-only audit.log                              │
│  · Future SaaS: OAuth2 + TLS sync layer (not enabled)   │
└─────────────────────────────────────────────────────────┘
```

**Cybersecurity posture (Phase 1 desktop):**

- No cloud calls, no telemetry, no ML inference
- Catalog and corrections stay on disk under the user profile
- MFT scan requires Administrator (expected for raw volume access)
- User corrections are explicit opt-in rules — never silent relabeling
- PE import check: `cargo run --release --bin ntfs-catalog -- verify-no-network`

## CLI (power users)

```powershell
cargo run --release --bin ntfs-catalog -- scan --drive C --output catalog.ntfsbin
cargo run --release --bin ntfs-catalog -- classify --catalog catalog.ntfsbin
```

## Tests

```powershell
cargo test
```

## Uninstall

```powershell
& "C:\Program Files\EngineVault\uninstall.ps1"
```
