use zbus::{Connection, proxy};
use std::error::Error;

// Definiamo un proxy per l'interfaccia UPower Device
#[proxy(
    interface = "org.freedesktop.UPower.Device",
    assume_defaults = true
)]
trait UPowerDevice {
    #[zbus(property)]
    fn percentage(&self) -> zbus::Result<f64>;

    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== KEYCHRON UPOWER BATTERY READER (FEDORA) ===");

    let connection = Connection::system().await?;

    // Percorso esatto trovato da UPower
    let path = "/org/freedesktop/UPower/devices/battery_hid_dco2co26o35o96o4d_battery";

    let device_proxy = UPowerDeviceProxy::builder(&connection)
        .path(path)?
        .destination("org.freedesktop.UPower")?
        .build()
        .await?;

    match device_proxy.percentage().await {
        Ok(level) => {
            let model = device_proxy.model().await.unwrap_or_else(|_| "Tastiera Wireless".to_string());
            println!("🔋 Dispositivo: {}", model);
            println!("   Livello batteria: {:.0}%\n", level);
        }
        Err(e) => {
            eprintln!("Impossibile leggere la percentuale della batteria: {}", e);
        }
    }

    Ok(())
}