use hidapi::HidApi;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== KEYCHRON VENDOR-DEFINED ACTIVE POLL (Col05) ===");

    let hid_api = HidApi::new()?;
    let mut target_device = None;

    for device in hid_api.device_list() {
        if device.vendor_id() == 0x05ac
            && device.product_id() == 0x024f
            && device.usage_page() == 65535
        {
            println!("⭐ Trovata interfaccia target: {:?}", device.path());
            if let Ok(dev) = device.open_device(&hid_api) {
                target_device = Some(dev);
                break;
            }
        }
    }

    let device = match target_device {
        Some(d) => d,
        None => {
            println!("Impossibile aprire l'interfaccia Vendor-Defined.");
            return Ok(());
        }
    };

    println!("✅ Connessione stabilita con la porta custom! Invio dei pacchetti di polling...\n");

    // Corretto l'uso di vec![ ... ] con le parentesi quadre
    let test_payloads: Vec<[u8; 64]> = vec![
        {
            let mut p = [0u8; 64];
            p[0] = 0x00;
            p[1] = 0x01;
            p
        },
        {
            let mut p = [0u8; 64];
            p[0] = 0x00;
            p[1] = 0x04;
            p[2] = 0x01;
            p
        },
        {
            let mut p = [0u8; 64];
            p[0] = 0x07;
            p
        },
        {
            let mut p = [0u8; 64];
            p[0] = 0x00;
            p[1] = 0xEE;
            p
        },
    ];

    for (i, payload) in test_payloads.iter().enumerate() {
        println!(
            "📤 Invio pacchetto di test #{}: {:02x} {:02x} {:02x}...",
            i + 1,
            payload[0],
            payload[1],
            payload[2]
        );

        if let Err(e) = device.write(payload) {
            println!("   ❌ Scrittura fallita: {}", e);
            continue;
        }

        let mut read_buf = [0u8; 64];
        let mut received_something = false;

        for _ in 0..3 {
            match device.read_timeout(&mut read_buf, 100) {
                Ok(n) if n > 0 => {
                    print!("   📥 RICEVUTO ({} bytes): ", n);
                    for byte in &read_buf[..n] {
                        print!("{:02x} ", byte);
                    }
                    println!();
                    received_something = true;
                }
                _ => {}
            }
        }

        if !received_something {
            println!("   ⏳ Nessuna risposta immediata a questo pacchetto.");
        }

        thread::sleep(Duration::from_millis(200));
    }

    println!("\n🔍 Test di polling completato. Rimaniamo in ascolto per 5 secondi...");

    let mut buf = [0u8; 64];
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if let Ok(n) = device.read_timeout(&mut buf, 500) {
            if n > 0 {
                print!("📥 [STREAM] ({} bytes): ", n);
                for b in &buf[..n] {
                    print!("{:02x} ", b);
                }
                println!();
            }
        }
    }

    Ok(())
}
