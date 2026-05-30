use std::io;

struct productos {
    nombre: String,
    precio: f64,
    numero_porciones: i32,
    cantidad_consumidores: i32,
}

impl productos {
    fn cantidad_necesaria_del_producto(&self) -> i32 {
        let mut cantidad_producto: i32 = 0;
        
        // Comparamos valores directamente, no referencias
        if self.cantidad_consumidores > self.numero_porciones {
            loop {
                cantidad_producto += 1;
                // Calculamos el total con la cantidad actual
                let cantidad_final = self.numero_porciones * cantidad_producto;
                
                // Si ya cubrimos o superamos a los consumidores, salimos
                if cantidad_final >= self.cantidad_consumidores {
                    break;
                }
            }
        } else {
            cantidad_producto = 1;
        }
        cantidad_producto
    }
}
pub fn adso_cotizacion_comida(){

}