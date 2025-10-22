// src-tauri/src/commands/auth_commands.rs
use crate::app_state::AppState;
use crate::core::error::AppError;

// Comando para login
#[tauri::command]
pub async fn login(
    user_id: String,
    state: tauri::State<'_, AppState>
) -> Result<String, AppError> {
    // Simulamos la lógica de login: obtener conexión y validar.
    // Aunque no se usa la conexión aún, la inyectamos para demostrar la arquitectura.
    let conn = state.db.lock().map_err(|_| AppError::DbLockError)?;
    
    // Lógica Mock: siempre exitoso
    println!("Login exitoso para usuario: {}", user_id);
    // En una implementación real se devolvería la struct User [cite: 29]
    Ok("token_de_usuario_simulado".to_string())
}

// Comando para obtener el usuario actual (mock)
#[tauri::command]
pub async fn get_current_user() -> Result<String, AppError> {
    // Devolvería el usuario actual del estado
    Ok("Usuario de Prueba".to_string())
}