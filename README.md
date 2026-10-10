# Keychron BLE Battery & Telemetry Tool (Rust / Linux / Hyprland)

An open-source, high-performance Rust utility designed to retrieve, monitor, and display battery telemetry for **non-QMK Keychron keyboards** (such as the K10, K2, and older classic K-series models) running on Linux (Fedora/Wayland/Hyprland environments via Bluetooth BLE).

Created as a reverse engineering challenge to overcome Windows driver restrictions, map device limitations, and bring native wireless battery monitoring to Linux.

---

## 🔍 The Investigation & Findings

If you are poking around the firmware of a standard, non-QMK Keychron keyboard (like the original K10 with VID/PID `0x05ac:0x024f`), here is what multi-interface sniffing and polling tests reveal:

1. **The USB Dead End:**
   - Standard typing interfaces (`MI_00`, `MI_01`) return `Access is denied` because the OS input driver locks them for keystrokes.
   - The custom vendor-defined interface (`Usage Page: 65535` / `0xFF00`) yields `Incorrect function` when attempting raw feature report polling or output writes.
   - **Conclusion:** On standard non-QMK models, the USB firmware does not implement host-side telemetry commands or write routines. Over a wired cable, it acts strictly as a standard HID device.

2. **The BLE & D-Bus Breakthrough:**
   - Battery reporting and telemetry are handled natively through the **Bluetooth (BLE)** stack.
   - On Linux (Fedora with BlueZ and UPower), once paired and configured, the system exposes the keyboard's battery status dynamically via D-Bus under `/org/freedesktop/UPower/devices/battery_hid_...`.

---

## 🛠️ Features

- **Direct D-Bus Integration:** Queries system services (`UPower`) asynchronously via `zbus` to fetch accurate battery percentages and power states.
- **CLI Mode with Continuous Monitoring:** Single-shot check or live-updating watch mode (`--watch`) with configurable intervals.
- **Waybar JSON Exporter (`--waybar`):** Instantly formats status and custom critical states for integration into status bars.
- **Native GUI (`--gui`):** Minimalist floating application built with `iced`, designed to look right at home on modern Wayland window managers like Hyprland.

---

## 🚀 Getting Started & Installation

### Prerequisites
- **Linux** (Tested on Fedora)
- **Rust & Cargo** ([rustup.rs](https://www.rust-lang.org/))
- Keyboard connected via **Bluetooth** (Ensure `Experimental = true` is set under `[General]` in `/etc/bluetooth/main.conf`).

### Build from Source
Clone the repository, compile the project, and build the optimized release binary:

```bash
git clone [https://github.com/GonnyTech/Keychron-Reverse-Engineering-Tool.git](https://github.com/GonnyTech/Keychron-Reverse-Engineering-Tool.git)
cd Keychron-Reverse-Engineering-Tool
cargo build --release

```

---

## 💻 Usage

1. **Quick Status Check (CLI):**
```bash
cargo run --release

```


2. **Continuous Monitoring:**
```bash
cargo run --release -- --watch --interval 30

```


3. **Launch the Native Floating GUI (Hyprland/Wayland):**
```bash
cargo run --release -- --gui

```


4. **Export Waybar JSON:**
```bash
cargo run --release -- --waybar

```



---

## ⚙️ Hyprland & Waybar Integration

To have your Keychron battery status displayed directly on your Waybar status bar with interactive click-to-open GUI support:

### 1. Configure Waybar (`config.jsonc` or `config_bottom.jsonc`)

Add the custom module inside your modules list (e.g., `modules-right`) and configure its execution path, interval, and click action:

```jsonc
"custom/keychron": {
    "exec": "/home/YOUR_USERNAME/Keychron-Reverse-Engineering-Tool/target/release/keychron_ble_battery --waybar",
    "return-type": "json",
    "interval": 30,
    "format": "⌨️  {}",
    "tooltip": true,
    "on-click": "/home/YOUR_USERNAME/Keychron-Reverse-Engineering-Tool/target/release/keychron_ble_battery --gui"
}

```

### 2. Style your Waybar (`style.css`)

Ensure the module matches your theme's typography and supports critical battery warning animations:

```css
/* Include the module in your main font/padding rules */
#custom-keychron {
    padding: 1px 5px 0 5px;
    color: #FFFFFF;
}

#custom-keychron.critical:not(.charging) { 
    color: #f53c3c; 
    animation-name: blink; 
    animation-duration: 1s; 
    animation-timing-function: linear; 
    animation-iteration-count: infinite; 
    animation-direction: alternate; 
} 

```

---

## 🤝 Contributions

Feel free to fork, open issues, or submit pull requests if you want to expand support to other classic Keychron models or add new features!
Questo file è pronto per essere salvato come `README.md` nella root del tuo progetto ed è perfetto per descrivere ogni singolo aspetto tecnico e pratico del lavoro svolto!
