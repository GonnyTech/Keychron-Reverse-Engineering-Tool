use std::error::Error;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let waybar_mode = args.iter().any(|arg| arg == "--waybar");
    let watch_mode = args.iter().any(|arg| arg == "--watch");
    let gui_mode = args.iter().any(|arg| arg == "--gui");

    if gui_mode {
        println!("Avvio GUI...");
        return Ok(());
    }

    if watch_mode {
        let mut last_percentage: u8 = 0;
        let mut last_update = Instant::now();

        loop {
            match read_upower_battery().await {
                Ok(battery) => {
                    last_percentage = battery;
                    last_update = Instant::now();
                    print_output(last_percentage, Some(last_update), waybar_mode);
                }
                Err(_) => {
                    if last_percentage > 0 {
                        print_output(last_percentage, Some(last_update), waybar_mode);
                    } else {
                        print_error(waybar_mode);
                    }
                }
            }
            sleep(Duration::from_secs(30)).await;
        }
    } else {
        match read_upower_battery().await {
            Ok(battery) => print_output(battery, Some(Instant::now()), waybar_mode),
            Err(_) => print_error(waybar_mode),
        }
    }

    Ok(())
}

/// Trova dinamicamente il path UPower della tastiera ed estrae la percentuale
async fn read_upower_battery() -> Result<u8, Box<dyn Error>> {
    let connection = Connection::system().await?;
    
    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        "org.freedesktop.UPower",
    ).await?;

    let devices: Vec<zbus::zvariant::OwnedObjectPath> = proxy.call("EnumerateDevices", &()).await?;

    for dev_path in devices {
        let path_str = dev_path.as_str();
        if path_str.contains("hid") && (path_str.contains("ee_74_0e") || path_str.contains("wearable") || path_str.contains("battery")) {
            let dev_proxy = zbus::Proxy::new(
                &connection,
                "org.freedesktop.UPower",
                path_str,
                "org.freedesktop.UPower.Device",
            ).await?;

            let percentage_val: zbus::zvariant::Value = dev_proxy.get_property("Percentage").await?;
            
            let percentage = match percentage_val {
                zbus::zvariant::Value::F64(v) => v as u8,
                zbus::zvariant::Value::U32(v) => v as u8,
                zbus::zvariant::Value::I32(v) => v as u8,
                _ => continue,
            };

            return Ok(percentage);
        }
    }

    Err("Dispositivo UPower della tastiera non trovato".into())
}

fn print_output(battery: u8, last_update: Option<Instant>, waybar_mode: bool) {
    let elapsed = last_update.map(|t| t.elapsed()).unwrap_or(Duration::from_secs(0));
    
    let staleness_str = if elapsed.as_secs() < 60 {
        "ora".to_string()
    } else if elapsed.as_secs() < 3600 {
        format!("{}m fa", elapsed.as_secs() / 60)
    } else {
        format!("{}h fa", elapsed.as_secs() / 3600)
    };

    if waybar_mode {
        let status_class = if battery <= 15 { "critical" } else { "normal" };
        println!(
            "{{\"text\": \"{}%\", \"percentage\": {}, \"class\": [\"{}\"], \"tooltip\": \"Keychron K10: {}% (Aggiornato: {})\"}}",
            battery, battery, status_class, battery, staleness_str
        );
    } else {
        println!("Keychron Battery: {}% (Aggiornato {})", battery, staleness_str);
    }
}

fn print_error(waybar_mode: bool) {
    if waybar_mode {
        println!("{{\"text\": \"N/A\", \"class\": [\"critical\"], \"tooltip\": \"Keychron Disconnected\"}}");
    } else {
        eprintln!("Errore nel recupero della batteria.");
    }
}