// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rexif::parse_file;
use tauri::command;

/// Lee los metadatos EXIF de una imagen y los devuelve como texto
#[command]
fn leer_exif(path: String) -> Result<Vec<(String, String)>, String> {
    let exif = parse_file(&path).map_err(|e| format!("Error leyendo EXIF: {}", e))?;

    let mut datos = Vec::new();

    for entry in exif.entries {
        datos.push((format!("{:?}", entry.tag), format!("{}", entry.value)));
    }

    println!("Archivo recibido: {}", path);
    Ok(datos)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![leer_exif])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
