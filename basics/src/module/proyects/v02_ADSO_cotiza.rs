use std::io;

struct ProductoInformacion{
    nombre          : String,
    precio_unitario : f64,
    porciones       : i32
}
impl ProductoInformacion{
}

fn menu() {
    let mut menu_completo = format!{
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
        menu();
        match io::stdin().read_line(&mut entrada_menu) {
            Ok(_) => {},
            Err(_) => {entrada_menu = "r232"},
        }

    }
}