// src-tauri/src/app_state.rs
use std::sync::Mutex;
use crate::db::connection::DbConnection;

// Estructura que define el estado global inyectado por Tauri 
pub struct AppState {
    // La conexión a la DB está envuelta en un Mutex para acceso concurrente seguro 
    pub db: Mutex<DbConnection>,
    // Otros estados podrían ir aquí (ej. la configuración del usuario). [cite: 43]
}