use std::{fmt::format, io, sync::BarrierWaitResult};

struct productos {
    nombre: String,
    precio: f64,
    numero_porciones: i32,
    cantidad_consumidores: i32,
}
impl productos {

    fn cantidad_necesaria_del_producto(&self) -> i32 {
        let mut cantidad_producto: i32 = 0;
        
        // Comparamos valores directamente
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

    //solicitamos datos
    let mut contador : i32 = 0;
    let mut opcion_01:productos = productos { 
            nombre:format!("hola"),
            precio: 0.0,
            numero_porciones: 0,
            cantidad_consumidores: 0, 
        };
    let mut opcion_02:productos = productos { 
            nombre:format!("hola"),
            precio: 0.0,
            numero_porciones: 0,
            cantidad_consumidores: 0, 
        };
    loop {
        // entradas pro

        let mut entrada_nombre:String =String::new();
        let mut entrada_precio:String =String::new();
        let mut entrada_porciones: String =String::new();
        let mut entrada_consumidores: String =String::new();


        contador += 1;
        println!("------- COTIZACION {} --------",contador);
        println!("Ingresa el nombre del Producto");
        io::stdin().read_line(&mut entrada_nombre).expect("error en la lectura del nombre");
        println!("Ingresa el precio del Producto");
        io::stdin().read_line(&mut entrada_precio).expect("error en la lectura del precio");
        println!("Ingresa el numero de porciones del Producto");
        io::stdin().read_line(&mut entrada_porciones).expect("error en la lectura del numero de porciones");
        println!("Ingresa el la cantidad de consumidores del Producto");
        io::stdin().read_line(&mut entrada_consumidores).expect("error en la lectura de los consumidores");

        // limpiamos y transformamos los datos
        if opcion_01.precio == 0.0 {
            opcion_01 = productos { 
                nombre: entrada_nombre.trim().to_string(), 
                precio: entrada_precio.trim().parse().expect("error al convertir precio"), 
                numero_porciones: entrada_porciones.trim().parse().expect("error al convertir precio"), 
                cantidad_consumidores: entrada_consumidores.trim().parse().expect("error al convertir precio"),  
            };
        } else {
            opcion_02 = productos { 
                nombre: entrada_nombre.trim().to_string(), 
                precio: entrada_precio.trim().parse().expect("error al convertir precio"), 
                numero_porciones: entrada_porciones.trim().parse().expect("error al convertir precio"), 
                cantidad_consumidores: entrada_consumidores.trim().parse().expect("error al convertir precio"),  
            };
            break;
        }
    }
    // imprime la facturafacturacion

    let mensaje_factura_01 = format!(
        "\n------FACTURA------\n\
        nombre   : {}\n\
        precio   : {}\n\
        cantidad : {}\n\n\
        primer total : {}\n",
        opcion_01.nombre,
        opcion_01.precio,
        opcion_01.cantidad_necesaria_del_producto(),
        opcion_01.costo_total()
    );

    let mensaje_factura_02 = format!(
        "\n------FACTURA------\n\
        nombre   : {}\n\
        precio   : {}\n\
        cantidad : {}\n\n\
        segundo total : {}\n",
        opcion_02.nombre,
        opcion_02.precio,
        opcion_02.cantidad_necesaria_del_producto(),
        opcion_02.costo_total()
    );


    println!("{}{}", mensaje_factura_01,mensaje_factura_02);

    
}