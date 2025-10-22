// src-tauri/src/commands/auth_commands.rs (CORREGIDO)
use crate::app_state::AppState;
use crate::core::error::AppError;

// Comando para login
#[tauri::command]
pub async fn login(
    user_id: String,
    state: tauri::State<'_, AppState>
) -> Result<String, AppError> {
    // La variable ahora es '_conn' para indicar que es intencionadamente no usada.
    let _conn = state.db.lock().map_err(|_| AppError::DbLockError)?;
    
    println!("Login exitoso para usuario: {}", user_id);
    // En una implementación real se devolvería la struct User
    Ok("token_de_usuario_simulado".to_string())
}

// Comando para obtener el usuario actual (mock)
#[tauri::command]
pub async fn get_current_user() -> Result<String, AppError> {
    // Devolvería el usuario actual del estado
    Ok("Usuario de Prueba".to_string())
}