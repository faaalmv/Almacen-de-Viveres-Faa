// src-tauri/src/core/error.rs (CORREGIDO Y ALINEADO AL BLUEPRINT)
use serde::Serialize;
use std::fmt;

// Le indicamos al compilador que ignore las advertencias de "código muerto" para este enum,
// ya que las otras variantes se usarán en el futuro según el Blueprint.
#[allow(dead_code)] 
#[derive(Debug, Serialize, Clone)]
pub enum AppError {
    // Errores de la capa de DB
    DbLockError,
    DbQueryError(String),
    // Errores de la lógica de negocio
    ValidationError(String),
    Unauthorized(String),
    // Errores generales
    InternalError,
}

// Implementación de Display para mostrar el error al usuario o loguear
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DbLockError => write!(f, "No se pudo obtener el bloqueo de la base de datos."),
            AppError::DbQueryError(msg) => write!(f, "Error en la consulta de la base de datos: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Error de validación: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Acceso no autorizado: {}", msg),
            AppError::InternalError => write!(f, "Un error interno desconocido ha ocurrido."),
        }
    }
}

// Implementación de From para convertir tauri::Error en AppError
impl From<tauri::Error> for AppError {
    fn from(_: tauri::Error) -> Self {
        AppError::InternalError
    }
}