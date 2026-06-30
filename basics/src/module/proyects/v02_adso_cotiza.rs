use std::{io, ops::ControlFlow::Continue};

struct ProductoInformacion{
    nombre          : String,
    precio_unitario : f64,
    porciones       : i32
}
impl ProductoInformacion{
}

fn menu_completo() {
    let menu = format!{
        "        (A). Ingresar un nuevo producto
        (B). Agregar Parametros Adicionales
        (C). Mostrar Factura
        (D). Limpiar
        (E). Salir
        "
    };
    println!("{menu}");
}




pub fn cotizacion(){
    //Creamos la entrada para los datos que da el usuario

    let mut entrada_menu : String = String::new();
    let mut entrada_producto_nombre : String = String::new();
    let mut entrada_producto_precio : String = String::new();
    let mut entrada_producto_porciones : String = String::new();


    //vector en el cual guardamos todos los productos 

    let mut base_datos_temporal:Vec<ProductoInformacion> =Vec::new();
    println!("!!!!Bienvenido A cotizador ADSO!!!!");

    loop {
        // Limpiamos el contenido de las variables

        entrada_menu.clear();
        entrada_producto_nombre.clear();
        entrada_producto_precio.clear();
        entrada_producto_porciones.clear();

        // Imprimimos el menu y solicitamos la opcion que desea el usuario 

        menu_completo();
        if let Err(e) =  io::stdin().read_line(&mut entrada_menu){
            eprintln!("error al recibir la entrada del menu");
            continue;
        }
        let entrada:&str = &entrada_menu.trim().to_uppercase();
        match &entrada as &str {
            "A" => {

                // Solicitamos los datos necesarios del producto
                loop {
                    entrada_producto_nombre.clear();
                    entrada_producto_precio.clear();
                    entrada_producto_porciones.clear();
                    
                    println!("Ingrese el nombre del producto");
                    if let Err(e) = io::stdin().read_line(&mut entrada_producto_nombre){
                        eprintln!("error al recibir la entrada del menu");
                        panic!();
                    };
                    

                    println!("ingrese el precio del producto");
                    if let Err(e) = io::stdin().read_line(&mut entrada_producto_precio){
                        eprintln!("error al recibir la entrada del menu");
                        panic!();
                    };

                    println!("Ingrese el numero de porciones del producto");
                    if let Err(e) = io::stdin().read_line(&mut entrada_producto_porciones){
                        eprintln!("error al recibir la entrada del menu");
                        panic!();
                    };

                    let mut entrada_base_datos : ProductoInformacion = ProductoInformacion { 
                        nombre: entrada_producto_nombre.trim().to_string(), 
                        precio_unitario: entrada_producto_precio.trim().parse().expect("precio unitario fallido intentelo nuevamente"), 
                        porciones: entrada_producto_porciones.trim().parse().expect(""), 
                    };

                    //Ingresamos el producto a la base de datos 

                    base_datos_temporal.push(entrada_base_datos);

                    //rompermos o continuamos con el ciclo

                    println!("Desea ingresar mas productos?(S o N)");
                    let mut entrada_opcion: String = String::new();
                    if let Err(e) = io::stdin().read_line(&mut entrada_opcion){
                        eprintln!("error al recibir la entrada del menu");
                        panic!();
                    };

                    // estudiar a mas profuncidad
                    let opcion_elegida = entrada_opcion.trim().to_uppercase().chars().next();

                    // 3. Evaluamos la opción usando Dust Matching o un If Let
                    if let Some('N') = opcion_elegida {
                        println!("Saliendo del ingreso de productos...");
                        // Aquí iría un 'break;' si estás dentro de un loop
                    } else if let Some('S') = opcion_elegida {
                        println!("Continuando...");
                    } else {
                        println!("Opción no válida, se asumirá que no deseas continuar.");
                    }


                }


            },
            "B" => println!("funciona la opcion B"),
            "C" => println!("funciona la opcion C"),
            "D" => println!("funciona la opcion D"),
            "E" => {
                println!("Hasta la proximaaaa.....");
                break;
            },
             _  => {println!("opcion no valida intentalo nuevamente");
            continue;},
            
        }
    }
}