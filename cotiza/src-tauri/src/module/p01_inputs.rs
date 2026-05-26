// Necesitamos importar esto para que Rust pueda transformar el JSON de JS en nuestro Struct
use serde::Deserialize;

#[derive(Deserialize)] // Le dice a Rust que este struct se puede crear a partir de datos externos (JS)
pub struct ItemCotizacion {
    nombre: String,
    cantidad: u32,
    precio: f64,
}
// basica mente el mesaje de abajo es un end poind
// osea hace que la funcion pueda ser llamada desde el frontend
#[tauri::command]
pub fn calcular_cotizacion(item: ItemCotizacion) -> String {
    let total = item.cantidad as f64 * item.precio;
    
    // Retornamos un mensaje de vuelta al frontend usando los campos del struct
    format!(
        "Cotización para '{}': {} unidades a ${:.2} c/u. Total: ${:.2}",
        item.nombre, item.cantidad, item.precio, total
    )
}

