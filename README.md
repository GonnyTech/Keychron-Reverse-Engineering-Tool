# Keychron K10 USB/HID Reverse Engineering Tool (Rust)

An experimental, low-level Rust utility designed to inspect, map, and communicate with **Keychron K10** keyboards (non-QMK/VIA version, standard Bluetooth/Wireless model) via USB HID on Windows, with ongoing exploration shifting towards Linux and Bluetooth (BLE).

Created as a personal reverse engineering challenge to explore device interfaces, Windows driver limitations, and firmware telemetry.

---

## 🔍 What We Discovered (The Findings)

If you are poking around the non-QMK Keychron K10 firmware, here is what our multi-interface sniffing and polling tests revealed:

1. **Hardware Identification:** 
   - VID/PID: `0x05ac:0x024f`
   - When switched to Mac mode, the keyboard identifies natively as an Apple peripheral, utilizing standard Apple descriptor templates.
2. **Multiple HID Collections:** 
   - The device exposes multiple interfaces and collections (`MI_00`, `MI_01`, spanning `Col01` through `Col05`).
   - Standard typing interfaces (`/KBD`, `MI_00`, `MI_01`) return `Access is denied (0x00000005)` because Windows' native input driver locks them exclusively for standard keystroke ingestion.
3. **The Vendor-Defined Interface (`65535` / `0xFF00`):**
   - We successfully isolated `Col05`, which features a custom `Usage Page: 65535` (Vendor-Defined). This is typically the backdoor used by proprietary vendor software.
4. **The Firmware Limitation:**
   - Writing raw output reports or testing feature report polling on the vendor-defined interface yields `Incorrect function (0x00000001)`. 
   - **Conclusion:** On the standard non-QMK K10 model, the USB firmware does not implement host-side telemetry commands or write routines for battery status/LEDs. It acts strictly as a standard HID input device over cable.

---

## What Next?

However, that doesn't mean it's a dead end! 

While the wired USB interface proves to be a telemetry blackout on non-QMK models, community documentation for older K-series boards (like the K2 and K6) indicates that battery reporting and device states are handled through the **Bluetooth (BLE)** stack rather than USB (which makes sense, considering that the battery voltage information is useful only in Bluetooth Mode). 

Since Windows hides that information behind closed system protocols, the current development phase is shifting to **Linux (Fedora) and BlueZ / D-Bus integration**, utilizing Rust (`zbus`) to directly query the `org.bluez.Battery1` interface when connected wirelessly.

---

## 🛠️ Code Structure

The repository includes:
* **Multi-Interface Scanner:** Maps all HID collections, usage pages, and paths.
* **Omni-Sniffer:** Multithreaded listener trying to capture raw traffic across all open endpoints.
* **Active Poller:** Tests various report IDs and magic header payloads against the custom vendor-defined interface.

## 🚀 Getting Started

Make sure you have [Rust and Cargo](https://www.rust-lang.org/) installed. Connect your keyboard via a USB cable and ensure the physical switch is set to **Cable**.

```bash
git clone [https://github.com/GonnyTech/Keychron-Reverse-Engineering-Tool.git](https://github.com/GonnyTech/Keychron-Reverse-Engineering-Tool.git)
cd Keychron-Reverse-Engineering-Tool
cargo run
```

---

## 🤝 Contributions & Continuation
Feel free to fork this repository, open issues, or test it on other non-QMK Keychron models. If you find a way to unlock the firmware telemetry via BLE or USB, pull requests are more than welcome!
