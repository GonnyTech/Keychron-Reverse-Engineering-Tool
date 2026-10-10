use zbus::{Connection, proxy};
use std::error::Error;
use clap::Parser;
use serde::Serialize;
use iced::widget::{button, column, text, container, Space};
use iced::{Element, Length, Sandbox, Settings};

#[proxy(
    interface = "org.freedesktop.UPower.Device",
    assume_defaults = true
)]
trait UPowerDevice {
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn state(&self) -> zbus::Result<u32>;
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Avvia la GUI grafica per Hyprland
    #[arg(short, long)]
    gui: bool,

    /// Esporta un file JSON formattato per Waybar nella cartella corrente
    #[arg(short, long)]
    waybar: bool,
}

#[derive(Serialize)]
struct WaybarOutput {
    text: String,
    tooltip: String,
    percentage: u8,
    class: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let connection = Connection::system().await?;
    let path = "/org/freedesktop/UPower/devices/battery_hid_dco2co26o35o96o4d_battery";

    let device_proxy = UPowerDeviceProxy::builder(&connection)
        .path(path)?
        .destination("org.freedesktop.UPower")?
        .build()
        .await?;

    let level = device_proxy.percentage().await.unwrap_or(0.0) as u8;
    let state = device_proxy.state().await.unwrap_or(0);
    let model = device_proxy.model().await.unwrap_or_else(|_| "Keychron K10".to_string());

    let state_str = match state {
        1 => "In carica ⚡",
        2 => "In scarica",
        4 => "Carica completa 🔌",
        _ => "Sconosciuto",
    };

    // Modalità Waybar JSON
    if args.waybar {
        let waybar_data = WaybarOutput {
            text: format!("{}%", level),
            tooltip: format!("{} - {} ({})", model, state_str, level),
            percentage: level,
            class: if level < 20 { "critical".into() } else { "normal".into() },
        };
        println!("{}", serde_json::to_string(&waybar_data)?);
        return Ok(());
    }

    // Modalità GUI Iced per Hyprland
    if args.gui {
        // Avviamo la nostra interfaccia grafica
        KeychronGuiApp::run(Settings::default())?;
        return Ok(());
    }

    // Modalità CLI classica di fallback
    println!("🔋 Dispositivo: {}", model);
    println!("   Livello: {}%", level);
    println!("   Stato: {}", state_str);

    Ok(())
}

// --- APPLICAZIONE GUI (ICED) ---
struct KeychronGuiApp {
    battery_level: u8,
    status: String,
}

#[derive(Debug, Clone)]
enum Message {
    Refresh,
    ExportWaybarJson,
}

impl Sandbox for KeychronGuiApp {
    type Message = Message;

    fn new() -> Self {
        Self {
            battery_level: 85, // Valore indicativo iniziale prima del fetch sincrono/asincrono
            status: "Pronto".into(),
        }
    }

    fn title(&self) -> String {
        "Keychron Battery Monitor".into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Refresh => {
                self.status = "Aggiornato!".into();
            }
            Message::ExportWaybarJson => {
                let waybar_data = WaybarOutput {
                    text: format!("{}%", self.battery_level),
                    tooltip: "Keychron K10 Battery".into(),
                    percentage: self.battery_level,
                    class: "normal".into(),
                };
                if let Ok(json) = serde_json::to_string(&waybar_data) {
                    let _ = std::fs::write("keychron_waybar.json", json);
                    self.status = "JSON salvato in keychron_waybar.json!".into();
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Keychron K10 Battery").size(24);
        let level_text = text(format!("{}%", self.battery_level)).size(48);
        let status_text = text(&self.status).size(14);

        let btn_refresh = button(text("Aggiorna").size(14)).on_press(Message::Refresh);
        let btn_json = button(text("Esporta JSON Waybar").size(14)).on_press(Message::ExportWaybarJson);

        let content = column![
            title,
            Space::with_height(20),
            level_text,
            status_text,
            Space::with_height(30),
            btn_refresh,
            btn_json,
        ]
        .padding(20)
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}