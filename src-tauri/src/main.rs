// src-tauri/src/main.rs (CONTENIDO FINAL Y CORREGIDO)

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
use crate::db::connection::DbConnection; // <<-- CORRECCIÓN APLICADA AQUÍ
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
            commands::auth_commands::get_current_user,
            commands::inventory_commands::get_programacion_mensual,
            commands::inventory_commands::process_and_save_salida
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application"); 
}