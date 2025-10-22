// src-tauri/src/main.rs (CONTENIDO FINAL Y LIMPIO)

// Declaración de módulos de la arquitectura, siguiendo la sección 2.1 del Blueprint
mod app_state;
mod commands;
mod core;
mod db;
mod models;

// Uso de estructuras y funciones de nuestros módulos
use app_state::AppState;
use db::connection::DbConnection;
use std::sync::Mutex; 

fn main() {
    // Inicializa la conexión a SQLite
    let db_connection = DbConnection::init();

    tauri::Builder::default()
        // Pasa la instancia de AppState a Tauri para gestión global
        .manage(AppState { db: Mutex::new(db_connection) })
        // Registra los comandos del backend
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::login,
            commands::inventory_commands::get_programacion_mensual,
        ])
        // LA LÍNEA DE PLUGINS NO ESENCIALES HA SIDO ELIMINADA DE AQUÍ
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); 
}