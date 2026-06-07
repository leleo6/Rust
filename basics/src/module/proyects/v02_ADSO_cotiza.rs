use std::io;

struct ProductoInformacion{
    nombre          : String,
    precio_unitario : f64,
    porciones       : i32
}
impl ProductoInformacion{
}

fn menu_completo() {
    let menu = format!{
       "(A). Ingresar un nuevo producto\n
        (B). Agregar Parametros Adicionales\n
        (C). Mostrar Factura\n
        (D). Limpiar\n
        (E). Salir\n
        "
    };
}



pub fn cotizacion(){
    loop {
        let mut entrada_menu : String = String::new();
        println!("!!!!Bienvenido A cotizador ADSO!!!!");
        menu_completo();
        if let Err(e) =  io::stdin().read_line(&mut entrada_menu){
            eprintln!("error al recibir la entrada del menu");
            continue;
        }

    }
}