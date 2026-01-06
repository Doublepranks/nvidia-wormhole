# Nvidia Wormhole 🌌
> **Bend space-time... or at least your GPU fan curves.**

![Nvidia Wormhole Icon](assets/icon.png)

## 🚀 Why this exists?

Let's be real: controlling Nvidia fans on Linux can be a pain, especially if you've moved to the future that is **Wayland** or desktop environments like KDE Plasma 6 where legacy System Tray icons go to die.

I built **Nvidia Wormhole** because I wanted a "Set and Forget" solution. I don't want a permanent window open, I don't want a flaky tray icon. I want to configure my curve once, have a daemon handle the heavy lifting quietly in the background, and move on with my life.

If you want a lightweight, modern, and persistent fan controller that respects your "No Root GUI" policies, you've found it.

## ✨ Features

- **Set & Forget Architecture**: GUI is just for config. The heavy lifting is done by a background daemon.
- **Daemon Mode**: Run `nvidia-wormhole --daemon` to enforce your curve without any visible window.
- **Wayland Ready**: Zero reliance on X11 trays.
- **Modern UI**: "Premium Dark" aesthetic because your tools should look as good as your games.
- **Enhanced Telemetry**: Real-time monitoring of GPU Temp, **Usage**, and Fan Speed.
- **Multi-Fan Support**: One curve to rule them all (controls all fans on the GPU).
- **Auto-Setup**: Built-in permission handler (`pkexec`) to auto-configure `sudoers` for `nvidia-settings`.

## 🛠️ Installation

### Prerequisites
You need the NVIDIA proprietary drivers installed on your **host system**. We rely on:
- `nvidia-settings` (for fan control)
- `nvidia-smi` (for robust telemetry)
- `pkexec` (PolicyKit, usually pre-installed on most distros)

### Method 1: Flatpak (Recommended)

The Flatpak version works seamlessly on Wayland and properly integrates with your desktop.

```bash
# Install dependencies
flatpak install flathub org.freedesktop.Platform//24.08
flatpak install flathub org.freedesktop.Sdk//24.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//24.08

# Clone and build
git clone https://github.com/Doublepranks/nvidia-wormhole.git
cd nvidia-wormhole

# Generate cargo sources (requires Python)
pip install --user tomlkit aiohttp
wget https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json

# Build and install
flatpak-builder --user --install --force-clean build-dir com.github.doublepranks.nvidia-wormhole.yml

# Run
flatpak run com.github.doublepranks.nvidia-wormhole
```

### Method 2: Native Cargo Build

If you prefer a native build without Flatpak:

```bash
git clone https://github.com/Doublepranks/nvidia-wormhole.git
cd nvidia-wormhole
cargo build --release
./target/release/nvidia-wormhole
```

### Post-Install (The "Set and Forget" part)

Inside the app, just check the **"Start daemon on login"** box. That's it.
The app will create an autostart entry to run the daemon on login.

## 📦 Dependencies

This project is built with **Rust** 🦀 using the [Iced](https://github.com/iced-rs/iced) framework.

### Core Crates:
- `iced`: For the beautiful, responsive GUI.
- `sysinfo`: For system data.
- `serde` / `serde_json`: For bulletproof config persistence.
- `anyhow` / `log`: For when things go sideways.
- `dirs`: For finding your `~/.config` correctly.
- `regex`: For parsing driver outputs.
- `image`: Because we like pretty icons.

### System Requirements:
- `libssl-dev`
- `pkg-config`
- `freetype2` (for Iced text rendering)
- `cmake` (build dependency)

## ⚖️ License

**Software Livre (Free Software)** forever.
Licensed under the **GPL-3.0 License**. You are free to view, modify, undo, break, fix, and redistribute this code.

See [LICENSE](LICENSE) for more details.

## ☕ Support the Developer

Did this tool save your RTX 4090 from thermal throttling? Did it silence that jet-engine noise while you were just browsing the web?

If you'd like to say thanks (or fund my caffeine addiction so I can fix more bugs), feel free to drop a donation!

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=PWCZCAATEGK3Q)

**Connect with me:**
- 🐦 [X / Twitter](https://x.com/sampantojapa)
- ⭐ [Star this repo!](https://github.com/Doublepranks/nvidia-wormhole)

_"Compiling code turns coffee into software."_ — Anonymous

---

Made with ❤️ and Rust.
