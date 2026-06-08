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
    let mut entrada_menu : String = String::new();
    println!("!!!!Bienvenido A cotizador ADSO!!!!");
    loop {
        entrada_menu.clear();//limpia el contenido de la variable

        menu_completo();
        if let Err(e) =  io::stdin().read_line(&mut entrada_menu){
            eprintln!("error al recibir la entrada del menu");
            continue;
        }
        let entrada:&str = &entrada_menu.trim().to_uppercase();
        match &entrada as &str {
            "A" => println!("funciona la obcion A"),
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