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
    fn costo_total(&self) -> f64{
        self.cantidad_necesaria_del_producto() as f64 * self.precio
    }

}


pub fn adso_cotizacion_comida(){
    // entradas

    let mut entrada_nombre:String =String::new();
    let mut entrada_precio:String =String::new();
    let mut entrada_porciones: String =String::new();
    let mut entrada_consumidores: String =String::new();

    //solicitamos datos

    println!("-------COTIZACION--------");
    println!("Ingresa el nombre del Producto");
    io::stdin().read_line(&mut entrada_nombre).expect("error en la lectura del nombre");
    println!("Ingresa el precio del Producto");
    io::stdin().read_line(&mut entrada_precio).expect("error en la lectura del precio");
    println!("Ingresa el numero de porciones del Producto");
    io::stdin().read_line(&mut entrada_porciones).expect("error en la lectura del numero de porciones");
    println!("Ingresa el la cantidad de consumidores del Producto");
    io::stdin().read_line(&mut entrada_consumidores).expect("error en la lectura de los consumidores");

    // limpiamos y transformamos los datos
    let opcion_01:productos =productos { 
        nombre: entrada_nombre.trim().to_string(), 
        precio: entrada_precio.trim().parse().expect("error al convertir precio"), 
        numero_porciones: entrada_porciones.trim().parse().expect("error al convertir precio"), 
        cantidad_consumidores: entrada_consumidores.trim().parse().expect("error al convertir precio"),  
    };
    println!("cantidad de produtos a comprar : {}",opcion_01.cantidad_necesaria_del_producto());
    println!("precio total a pagar: {}",opcion_01.costo_total());

    
}