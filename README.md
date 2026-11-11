# ViaHorizonPlus

**English** | [Français](README-fr.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Minecraft: 26.3](https://img.shields.io/badge/Minecraft-26.3-brightgreen.svg)](https://playhorizon.dev/)
[![Horizon Native Mod](https://img.shields.io/badge/Horizon-Native%20Mod-blueviolet.svg)](https://playhorizon.dev/mods)
[![CI & Build](https://github.com/rraattrruuee/ViaHorizonPlus/actions/workflows/ci.yml/badge.svg)](https://github.com/rraattrruuee/ViaHorizonPlus/actions)

**ViaHorizonPlus** is a native **Rust** port and adaptation inspired by [ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus), built specifically for the [Horizon](https://playhorizon.dev/) client engine (a complete Minecraft rewrite in Rust with WGPU and Tokio).

It enables players on Horizon to connect to older Minecraft servers (from legacy 1.8.x up to modern 26.3) with an in-game protocol selector directly on the Multiplayer screen, along with legacy 1.8 combat and movement emulation.

---

## 🌟 Features

- **In-Game Protocol Selector**: Dropdown menu directly on the top-right of the Multiplayer screen.
- **Auto-Detection (Ping)**: Automatically negotiates the target server version or allows forcing a specific Minecraft version.
- **Protocol Support**: Connect to servers running Minecraft 1.8.x through 26.3.
- **Legacy 1.8 Gameplay Emulation**:
  - Right-click sword blocking
  - Removal of the 1.9+ attack delay / cooldown
  - Classic 1.7 / 1.8 block-hitting animation
  - Classic 1.8 sneaking eye height (1.54m instead of modern 1.27m)
- **100% Native Rust**: Zero Java JVM, Fabric, or bytecode manipulation needed. Injected at compile-time into the Horizon client AST.

---

## 🔍 Why a Dedicated Port for Horizon?

1. **No Java Virtual Machine (JVM) or Fabric**:  
   Horizon is written entirely in Rust. Traditional Java mods (`.jar`), mod loaders (Fabric, NeoForge), and bytecode mixin libraries cannot run inside Horizon.
2. **Native AST Injection (`horizon-mixins`)**:  
   Horizon uses native mods (`.hnmod`) compiled directly into the client binary through AST transformations defined in `horizon-mixin.toml`.
3. **Rust Protocol Translation**:  
   Packet encoding, decoding, handshake interception, and version adaptation are handled directly inside Horizon's network layer.

---

## 📁 Repository Structure

```
viahorizonplus/
├── bundle/mixins/viahorizonplus/    # Native mixin sources for Horizon
│   ├── horizon-mixin.toml          # Mixin manifest (hooks, modules, injections)
│   ├── runtime.rs                  # Version selector UI & runtime state
│   ├── older_server.rs             # Server version detection hook
│   ├── legacy_run.rs               # Network packet loop & adaptation
│   ├── screen.rs / click.rs        # UI rendering and input hooks
│   └── settings.rs                 # Mod settings integration
├── src/                            # Core Rust crate library
│   ├── protocol/                   # Protocol versions, Handshake, Translator traits
│   ├── emulation/                  # 1.8 combat, movement, and visuals emulation
│   └── gui/                        # In-game GUI components
├── .github/workflows/ci.yml        # Automated CI, testing & packaging workflow
├── compatibility.json              # Pinned Horizon engine & mixins revisions
├── icon.png                        # Official 256x256 mod icon
├── LICENSE                         # MIT License
└── NOTICE                          # Attribution notices
```

---

## 🚀 Building & Testing

### Running Rust Unit Tests:
```bash
cargo check
cargo test
```

### Testing with Horizon Launcher:
Launch Horizon in development mode with the mod:
```bash
horizon-launcher --dev-mod "/path/to/viahorizonplus"
```

### Packaging for the Horizon Registry:
Clone the official `horizon-mods` repository to package into a `.hnmod` bundle:
```bash
python3 native/tools/native.py package /path/to/viahorizonplus
cargo horizon-mod check target/viahorizonplus-0.1.5.hnmod
```

---

## 📜 License & Credits

Distributed under the **[MIT License](LICENSE)**.

### Credits & Attribution:
- Inspired by the concept of **[ViaFabricPlus](https://github.com/ViaVersion/ViaFabricPlus)** by **FlorianMichael** and the **ViaVersion** team.
- 100% native Rust implementation for Horizon created by **rraattrruuee** and contributors.
- Minecraft and related assets are trademarks of Mojang Synergies AB.
