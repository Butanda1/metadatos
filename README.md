Aplicación de escritorio multiplataforma ligera desarrollada con **Tauri v2**, **Rust** y **TypeScript** para inspeccionar y extraer metadatos EXIF de archivos de imagen locales (`.jpg`, `.jpeg`, `.png`, `.tiff`, `.bmp`).

## Características Principales
* **Selección Nativa de Archivos:** Integración con el explorador del sistema operativo mediante `tauri-plugin-dialog`.
* **Procesamiento Rápido en Backend:** Lectura y análisis binario de etiquetas EXIF ejecutado directamente en Rust mediante la librería `rexif`.
* **Comunicación IPC Segura:** Intercambio de datos serializados (`serde_json`) entre el backend nativo y la interfaz web mediante comandos `invoke`.

## Stack Tecnológico
* **Framework de Escritorio:** Tauri v2
* **Backend Nativo:** Rust (Edición 2021), `rexif`, `serde` / `serde_json`, `tauri-plugin-dialog`, `tauri-plugin-log`
* **Frontend:** TypeScript, HTML5, CSS3, `@tauri-apps/api`
* **Herramientas de Compilación:** Vite, Cargo, npm

## Ejecución en Desarrollo
1. Instalar dependencias del frontend:
   ```bash
   npm install
