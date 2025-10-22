// src-tauri/src/main.rs (CONTENIDO FINAL Y CORRECTO)

// Previene que se abra una ventana de consola en Windows en modo release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Declaración de módulos de la arquitectura
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
    let db_connection = DbConnection::init();

    tauri::Builder::default()
        .manage(AppState { db: Mutex::new(db_connection) })
        .invoke_handler(tauri::generate_handler![
            commands::auth_commands::login,
            commands::auth_commands::get_current_user, // Añadido para que la UI de prueba funcione
            commands::inventory_commands::get_programacion_mensual
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); 
}