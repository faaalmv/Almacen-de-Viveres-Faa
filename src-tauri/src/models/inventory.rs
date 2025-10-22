// src-tauri/src/models/inventory.rs (CONTENIDO AUTO-CONTENIDO)
use serde::{Deserialize, Serialize};

// Representa un ítem en la orden de salida que viene del frontend.
// La directiva 'derive' le pide al compilador que genere automáticamente el código
// para Debug (imprimir), Clone (copiar), Deserialize (convertir de JSON a Struct)
// y Serialize (convertir de Struct a JSON).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderItem {
    pub id: String,
    pub description: String,
    #[serde(rename = "cantPedida")] // Mapea el camelCase de JS al snake_case de Rust
    pub cant_pedida: u32,
    #[serde(rename = "cantSurtida")]
    pub cant_surtida: u32,
}

// Representa el número de folio que se devolverá al frontend tras un guardado exitoso.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Folio {
    pub number: String,
}

impl Folio {
    // Una función simple para generar un nuevo número de folio (simulado).
    pub fn new() -> Self {
        // En una implementación real, este número se generaría de forma secuencial
        // o a partir de un algoritmo específico.
        Folio {
            number: "SAL-2025-001".to_string(),
        }
    }
}