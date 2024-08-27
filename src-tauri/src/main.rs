// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn list_serial_ports() -> Result<Vec<String>, String> {
    match serialport::available_ports() {
        Ok(ports) => {
            let port_names: Vec<String> = ports.iter().map(|p| p.port_name.clone()).collect();
            Ok(port_names)
        }
        Err(e) => Err(format!("Failed to list serial ports: {:?}", e)),
    }
}
fn read_from_serial(port_name: String, baud_rate: u32, window: tauri::Window) {
    let mut serial_settings: SerialPortSettings = Default::default();
    serial_settings.baud_rate = baud_rate;
    serial_settings.timeout = Duration::from_secs(2);

    let mut serial_port = match serialport::open_with_settings(&port_name, &serial_settings) {
        Ok(port) => port,
        Err(e) => {
            window
                .emit(
                    "serial-error",
                    format!("Error opening serial port: {:?}", e),
                )
                .unwrap();
            return;
        }
    };

    let mut buffer: Vec<u8> = vec![0; 1024];

    loop {
        match serial_port.read(&mut buffer) {
            Ok(bytes_read) => {
                let data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                window.emit("serial-data", data).unwrap(); // Emit event to frontend
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue, // Retry on timeout
            Err(e) => {
                window
                    .emit(
                        "serial-error",
                        format!("Error reading serial port: {:?}", e),
                    )
                    .unwrap();
                break;
            }
        }
    }
}

#[tauri::command]
fn start_serial_reading(port_name: String, window: tauri::Window) -> Result<(), String> {
    let baud_rate = 9600;
    thread::spawn(move || {
        read_from_serial(port_name, baud_rate, window);
    });
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            start_serial_reading
        ])
        .setup(|app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
