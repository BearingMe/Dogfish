# Skyrim Integration Environment Survey & Architecture Assessment

- **Document**: Technical Survey & Engine Boundary Evaluation
- **Milestone**: M48 — Skyrim Integration Environment Survey
- **Phase**: Phase 12 (Reconstruction Foundations)
- **Subject**: *Conquest of Skyrim* v0.3.9 Integration Paths
- **Investigation Date**: 2026-10-28
- **Status**: SPECIFIED & APPROVED

---

## 1. Executive Summary & Purpose

The purpose of this survey is to systematically investigate Skyrim Special Edition runtime integration options for later project phases:
- **Phase 13 (M49–M54)** implements the independent, standalone domain simulation model in pure Rust (`reconstruction/`), requiring zero engine dependencies.
- **Phase 15 (M59–M62)** designs and executes experimental Skyrim runtime integration, culminating in a minimal in-game vertical slice (M60) demonstrating bidirectional communication between Skyrim and the reconstructed simulation.

This document surveys target runtimes, plugin record structures, scripting and native bridge layers, state persistence models, UI frameworks, Creation Kit requirements, and available development toolchains to establish the optimal technical baseline for Phase 15.

---

## 2. Target Runtime & Binary Environment

### 2.1 Skyrim Special Edition Version Fragmentation

The 64-bit Skyrim Special Edition (SSE) ecosystem is divided across three primary runtime baselines:

| Runtime Version | Common Name | Release Period | Binary Characteristics & Considerations |
| :--- | :--- | :--- | :--- |
| **1.5.97.0** | SSE Pre-Anniversary Edition | 2019–2021 | The historical "golden standard" for SKSE plugins. Uses standard memory offsets documented in classic SKSE64. Fixed binary layout, but lacks official Bethesda Creations / Marketplace updates. |
| **1.6.640.0** | Skyrim Anniversary Edition (AE) | Sept 2022 | Widely adopted Steam/GOG baseline. Recompiled with Visual Studio 2019/2022; altered memory offsets and struct layouts across virtually all engine classes. |
| **1.6.1170.0+** | Modern AE / Steam Build | Dec 2023+ | Current public Steam build. Modifies internal FormID ranges and resource loaders. Supported by modern SKSE64 (v2.2.6+). |

### 2.2 Address Library & Runtime Agnosticism
- **Mechanics**: Modern SKSE plugins avoid hardcoding raw memory addresses by utilizing *Address Library for SKSE Plugins* (`versionlibdb.bin`).
- **Functionality**: Address Library maps standardized function IDs to version-specific memory offsets at runtime.
- **Recommendation for Phase 15**: Target **Skyrim Special Edition 1.6+ (AE 1.6.640 / 1.6.1170)** using Address Library IDs or version-independent hook frameworks (CommonLibSSE-NG), ensuring maximum modern compatibility without pinning to obsolete 1.5.97 binaries.

---

## 3. Plugin Architecture & Record Types

Skyrim mods inject records into the engine using one of three plugin packaging formats:

```text
                               Plugin Architecture Comparison
  ┌───────────────────────┬───────────────────────┬────────────────────────┐
  │      Standard ESP     │       Full ESM        │     Light ESL / ESP-FE │
  ├───────────────────────┼───────────────────────┼────────────────────────┤
  │ • Form Version 44     │ • Master Flag enabled │ • ESP with ESL Flag    │
  │ • Consumes 1 of 254   │ • Loads early in root │ • Consumes Slot 0xFE   │
  │   master slots        │ • Master load priority│ • Up to 4,096 plugins  │
  │ • Unlimited FormIDs   │ • Unlimited FormIDs   │ • Max 2,048 new forms  │
  │ • Conquest uses this  │ • Heavyweight         │   (0x800 - 0xFFF)      │
  └───────────────────────┴───────────────────────┴────────────────────────┘
```

### 3.1 Evaluation for Dogfish Integration
1. **Conquest of Skyrim Baseline**: `conquerskyrim.esp` is a standard Form Version 44 ESP consuming 1 active plugin slot (`0x00`–`0xFD`), defining 22,690 records.
2. **Reconstruction Minimal Slice (M60)**:
   - For an experimental vertical slice claiming a single test location or receiving game events, an **ESL-flagged ESP (`ESP-FE`)** is strongly recommended.
   - *Advantages*: Consumes slot `0xFE`, bypasses the 254-plugin ceiling, prevents load-order collisions, and isolates test records within the `0x800`–`0xFFF` FormID allocation window.

---

## 4. Scripting & Native Extension Layer Options

To connect Skyrim runtime events (e.g., actor activation, location change, combat death) to the reconstructed simulation model, several bridging layers were evaluated:

### 4.1 Comparison of Scripting & Extension Approaches

| Integration Approach | Latency & Performance | Safety & Isolation | Development Complexity | Tooling Dependencies |
| :--- | :--- | :--- | :--- | :--- |
| **Option A: Pure Vanilla Papyrus** | Poor (asynchronous, throttled to 1.2ms task slice) | High (engine sandbox) | High (verbose syntax, limited data structures) | Creation Kit / Papyrus Compiler |
| **Option B: Papyrus + SKSE Native Functions** | Moderate (Papyrus invocation overhead remains) | Moderate (C++ native crashes crash game) | Moderate (dual-language binding required) | Papyrus Compiler + MSVC / C++ |
| **Option C: CommonLibSSE C++ Plugin** | Excellent (direct engine thread execution) | Low (direct memory manipulation) | Moderate/High (reverse-engineered C++20 headers) | Visual Studio 2022, CMake, Ninja |
| **Option D: Standalone Rust DLL (`cdylib` via C-ABI)** | Excellent (sub-millisecond native execution) | High (Rust memory safety in simulation core) | Moderate (requires thin C-ABI SKSE entry point) | `cargo`, `rustc`, `cl.exe` |

### 4.2 Detailed Analysis of Native Extension Architectures

#### Architecture 1: Classic SKSE64 C++ Plugin
- Defined in pinned reference `references/sources/skse64`.
- Exports standard C functions:
  ```cpp
  extern "C" {
      __declspec(dllexport) bool SKSEPlugin_Query(const SKSEInterface* skse, PluginInfo* info);
      __declspec(dllexport) bool SKSEPlugin_Load(const SKSEInterface* skse);
  }
  ```
- *Limitation*: Requires maintaining version-specific memory offsets manually unless paired with Address Library.

#### Architecture 2: CommonLibSSE-NG (C++20)
- The modern gold standard for Skyrim SE native development.
- Provides complete reverse-engineered class hierarchies (`RE::TESForm`, `RE::TESObjectREFR`, `RE::PlayerCharacter`), event sinks (`BSTEventSink<RE::TESCombatEvent>`), and automated Address Library resolution across all 1.5.97, 1.6.640, and 1.6.1170+ runtimes.

#### Architecture 3: Pure Rust SKSE Plugin (`cdylib`)
- The SKSE loader communicates via a standard Windows DLL C-ABI:
  ```rust
  #[no_mangle]
  pub extern "C" fn SKSEPlugin_Query(
      skse: *const SKSEInterface,
      info: *mut PluginInfo,
  ) -> bool { ... }

  #[no_mangle]
  pub extern "C" fn SKSEPlugin_Load(
      skse: *const SKSEInterface,
  ) -> bool { ... }
  ```
- *Benefit*: Permits compiling the standalone Rust reconstruction model (`reconstruction/src/lib.rs`) directly into the SKSE native plugin as a dynamic library (`cdylib`), eliminating the need for an intermediate C++ bridge layer.

---

## 5. Savegame Persistence Strategies

How domain state survives across game sessions is critical to preventing savegame corruption and mod bloat:

```text
                                Persistence Strategy Matrix
  ┌────────────────────────────────────────────────────────────────────────┐
  │ Option 1: Vanilla Papyrus Script State (.ess)                          │
  │ • Serialized directly inside Skyrim .ess savegame.                     │
  │ • Risk: Orphaned script instances on uninstall; save bloat.            │
  ├────────────────────────────────────────────────────────────────────────┤
  │ Option 2: SKSE Co-Save (.skse)                                         │
  │ • Serialized alongside savegame in separate companion file.            │
  │ • Managed via SKSESerializationInterface (Save, Load, Revert hooks).   │
  │ • Clean: Removing mod does not corrupt core .ess binary.               │
  ├────────────────────────────────────────────────────────────────────────┤
  │ Option 3: External Decoupled State File (JSON / SQLite)                 │
  │ • Serialized to independent file on disk (e.g. saves/dogfish_state.json│
  │ • Tied to Skyrim save events via GameID / SaveUID token.               │
  │ • Zero engine footprint; fully inspectable and testable outside game.  │
  └────────────────────────────────────────────────────────────────────────┘
```

### Strategic Evaluation:
- **Conquest of Skyrim Approach**: Uses Option 1 (100% Papyrus script state), resulting in severe savegame bloat, risk of orphaned scripts, and mid-siege save desynchronization (`CS_AttackCityScript` lacks reload handlers).
- **Dogfish Recommendation for Phase 15**: Use **Option 2 (SKSE Co-Save) or Option 3 (Decoupled External JSON/SQLite)** synchronized through SKSE save/load callbacks. This guarantees zero save corruption while enabling independent offline state verification.

---

## 6. User Interface Layer Options

Conquest of Skyrim exposes its management systems via complex nested dialogue trees (935 `TopicInfo` records), message boxes, and a SkyUI MCM configuration panel.

| UI Technology | Platform / Runtime | Capabilities & Limitations | Development Dependencies |
| :--- | :--- | :--- | :--- |
| **Scaleform (Flash / AS2)** | Native Skyrim HUD / Menu (GFx) | Rich interactive menus (SkyUI inventory, custom full-screen maps). Difficult to author; requires legacy Flash / ActionScript 2.0 toolchains (JPEXS / ffdec). | ActionScript 2 compiler, Java, JPEXS |
| **Dialogue Trees (`DIAL`/`INFO`)** | Vanilla Dialogue Engine | Immersive, fully voiced (or silent subtitle), context-aware. Highly cumbersome for complex tabular data (e.g. army ledgers, treasury balances). | Creation Kit / xEdit |
| **Message Boxes (`MESG`)** | Vanilla Popups | Simple 1..9 button choices. Clean and stable, but cannot render dynamic lists or pagination cleanly. | Creation Kit / xEdit |
| **Dear ImGui Native Overlay** | DirectX 11 / DXGI Swapchain Hook | Rendered directly via SKSE DirectX hook. Exceptional for debug dashboards, real-time metrics, developer inspections, and test controls. | C++ / Rust DirectX 11 hook |

### UI Recommendation for Dogfish:
- For experimental research probing (Phase 15), **Dialogue/Message Boxes + Dear ImGui Debug Overlay** is the superior strategy.
- Avoids the fragile and obsolete Adobe Flash / ActionScript 2 toolchain while providing instant, high-fidelity visualization of simulation state (hold loyalty, troop distributions, economic reserves).

---

## 7. Creation Kit & Manual Authoring Requirements

Even with a native Rust simulation core, Skyrim engine integration requires specific native game records:

### 7.1 Mandatory Engine Records for Minimal Slice (M60)
1. **Quest Record (`QUST`)**:
   - Acts as the runtime anchor for event registration (`RegisterForSingleUpdate`, `RegisterForModEvent`).
   - Must be flagged *Start Game Enabled* or initialized via clean trigger.
2. **Test Location / Cell (`CELL` / `WRLD`)**:
   - An isolated interior test cell (e.g. `DogfishTestingRoom` cloned from `qasmoke`) is vastly superior to exterior worldspaces.
   - *Advantage*: Completely avoids complex exterior navmesh generation, worldspace LOD recalculations, and vanilla Civil War territory quest conflicts.
3. **Trigger References (`REFR`)**:
   - A single interactive test activator (e.g., a test banner or console activator) with a minimal attached Papyrus stub that dispatches a native event to the integration DLL.
4. **Keyword Definitions (`KYWD`)**:
   - Minimal holding keywords (`DogfishHoldingKeyword`) to identify claimable test points.

### 7.2 Tooling Feasibility for Record Creation
- **Creation Kit 2.0 (Official Bethesda Editor)**: Required for visual cell layout and navmesh generation. Heavyweight and prone to crashes.
- **xEdit / SSEEdit (Community Record Editor)**: Capable of creating 100% of the required `QUST`, `CELL`, `REFR`, and `KYWD` records programmatically or through structured manual GUI editing without running the Creation Kit.

---

## 8. Host Environment Tooling Inventory

An audit of the local development workstation was conducted to verify compiler, build tool, and runtime availability:

| Tool / Dependency | Detected Version | System Path | Status & Capability |
| :--- | :--- | :--- | :--- |
| **Rust Toolchain (`rustc`)** | `1.98.1` (48a229cea) | System PATH | **VERIFIED** — Ready for `reconstruction/` core and `cdylib` native plugin compilation. |
| **Rust Package Manager (`cargo`)**| `1.98.1` (797e8a9bc) | System PATH | **VERIFIED** — Supports multi-crate workspace, test harness, and FFI builds. |
| **Python Runtime** | `3.13.3` (64-bit) | System PATH | **VERIFIED** — Supports persistent scripts (`scripts/`) and automated test suites (`tests/`). |
| **Microsoft Visual C++ (MSVC)** | VS 2022 Community (`17.x`) | `C:\Program Files\Microsoft Visual Studio\2022\Community` | **VERIFIED** — Full C++20 compiler (`cl.exe`), Windows SDK, and linkers available. |
| **CMake** | `3.30+` | WinGet Local Package | **VERIFIED** — Available for C++ builds (SKSE64, CommonLibSSE). |
| **Ninja Build System** | `1.11+` | Python Scripts PATH | **VERIFIED** — High-speed parallel C/C++ compilation. |
| **SkyUI Reference Source** | `6.0+` Submodule | `references/sources/skyui-community` | **VERIFIED** — Reference available for UI bindings and ActionScript models. |
| **SKSE64 Reference Source** | Version Pinned | `references/sources/skse64` | **VERIFIED** — Reference available for native engine loader bindings. |
| **Conquest of Skyrim Local** | v0.3.9 Data (521.93 MB) | `references/local/conquest-of-skyrim/Data` | **VERIFIED** — 1,539 PSC scripts and `conquerskyrim.esp` available for probing. |

---

## 9. Integration Trade-Offs & Roadmap for Phase 15

Based on the survey findings, the recommended technical path for Phase 15 (M59–M62) is established:

```text
                               Recommended Integration Stack
  ┌────────────────────────────────────────────────────────────────────────┐
  │ 1. Engine Target: Skyrim SE / AE 1.6+ (Steam 1.6.640 / 1.6.1170)       │
  │    • Uses Address Library for runtime offset independence.             │
  ├────────────────────────────────────────────────────────────────────────┤
  │ 2. Plugin Record: ESL-Flagged ESP (ESP-FE)                             │
  │    • Slot 0xFE, FormIDs 0x800-0xFFF, zero load-order slot consumption. │
  ├────────────────────────────────────────────────────────────────────────┤
  │ 3. Core Engine Bridge: Pure Rust `cdylib` or Thin CommonLibSSE Proxy   │
  │    • Direct C-ABI export of SKSEPlugin_Load / SKSEPlugin_Query.        │
  │    • Embeds or links to `reconstruction` Rust simulation crate.        │
  ├────────────────────────────────────────────────────────────────────────┤
  │ 4. State Persistence: Decoupled JSON / SQLite via SKSE Co-Save Hooks   │
  │    • Zero savegame bloat; no orphaned Papyrus scripts on uninstall.    │
  ├────────────────────────────────────────────────────────────────────────┤
  │ 5. Test Environment: Isolated Interior Test Cell (e.g. DogfishTestRoom)│
  │    • Bypasses exterior worldspace navmesh and Civil War script traps.  │
  ├────────────────────────────────────────────────────────────────────────┤
  │ 6. Inspection UI: In-Game Dialogue/Activator + ImGui Telemetry Overlay │
  │    • Avoids legacy Adobe Flash / ActionScript 2 toolchain dependencies.│
  └────────────────────────────────────────────────────────────────────────┘
```

---

## 10. Conclusion & Phase 12 Gate Readiness

Milestone M48 establishes a rigorous, evidence-backed technical foundation for future Skyrim engine integration:
1. Target runtimes, record formats, and extension layers are clearly evaluated with trade-offs.
2. The workstation toolchain (Rust 1.98, MSVC 2022, CMake, Ninja, Python 3.13) is verified and ready.
3. Complex manual steps (Creation Kit records, interior cell isolation) are scoped to prevent downstream roadblocks.
4. Completes all deliverables for Phase 12 (Reconstruction Foundations: M45, M46, M47, M48).
