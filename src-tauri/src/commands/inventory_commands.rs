// src-tauri/src/commands/inventory_commands.rs
use crate::app_state::AppState;
use crate::core::error::AppError;

// Comando para obtener la programación mensual (mock)
#[tauri::command]
pub async fn get_programacion_mensual(
    month: u8,
    year: u16,
    state: tauri::State<'_, AppState>
) -> Result<Vec<String>, AppError> {
    // Simulamos un acceso a la base de datos para recuperar datos.
    let _conn = state.db.lock().map_err(|_| AppError::DbLockError)?;
    
    println!("Recuperando programación para {}/{}", month, year);
    
    // Lógica Mock: devuelve una lista simple de elementos
    Ok(vec![
        format!("Item 1 para {}/{}", month, year),
        format!("Item 2 para {}/{}", month, year),
    ])
}