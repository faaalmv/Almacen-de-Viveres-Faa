// src-tauri/src/commands/inventory_commands.rs (CONTENIDO AUTO-CONTENIDO)
use crate::app_state::AppState;
use crate::core::error::AppError;
// Importamos los modelos definidos en la misión anterior
use crate::models::inventory::{OrderItem, Folio}; 
use std::collections::HashMap; // Necesario para el mock de ProgramacionMensual

// Comando principal para procesar y guardar la salida (Sección 2.4 del Blueprint)
#[tauri::command]
pub async fn process_and_save_salida(
    items: Vec<OrderItem>,
    service: String,
    state: tauri::State<'_, AppState>
) -> Result<Folio, AppError> {
    // 1. Demostramos la inyección segura del estado de la DB
    let _conn = state.db.lock().map_err(|_| AppError::DbLockError)?;
    
    // 2. Simulamos la validación (ej. ¿cantSurtida es válida?)
    for item in &items {
        if item.cant_surtida > item.cant_pedida {
            // Utilizamos el error de validación definido en nuestro Blueprint
            return Err(AppError::ValidationError(format!(
                "Error en item {}: Cantidad surtida ({}) excede cantidad pedida ({})", 
                item.description, item.cant_surtida, item.cant_pedida
            )));
        }
    }

    // Lógica Mock: Simular las operaciones de DB (4, 5, 6 del Blueprint)
    println!("Procesando salida para servicio: {}", service);
    println!("Items a guardar: {:#?}", items);

    // 7. Devolver el folio generado al frontend.
    Ok(Folio::new())
}


// Comando para obtener la programación mensual (mock original)
#[tauri::command]
pub async fn get_programacion_mensual(
    month: u8,
    year: u16,
    state: tauri::State<'_, AppState>
) -> Result<Vec<String>, AppError> {
    let _conn = state.db.lock().map_err(|_| AppError::DbLockError)?;
    
    println!("Recuperando programación para {}/{}", month, year);
    
    // Lógica Mock: devuelve una lista simple de elementos
    Ok(vec![
        format!("Item 1 para {}/{}", month, year),
        format!("Item 2 para {}/{}", month, year),
    ])
}