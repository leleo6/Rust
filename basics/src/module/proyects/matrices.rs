
use std::io;

fn mostrar_menu(){
    println!("
        (A) incertar un numero
        (B) incentar un mensaje
        (C) Mostrar
        (D) Salir
    ")
}


pub fn meter_incices_mostar(){
    let mut entrada =String::new();
    let mut vector_numeros: Vec<f64> = vec![];
    let mut vector_mensaje: Vec<String> = vec![];
    let mut letra:char ='0';

    println!("Manejo de Vectores");
    loop {
        mostrar_menu();
        if let Err(e) = io::stdin().read_line(&mut entrada) {
            eprintln!("Error {}",e);
            continue;
        };
        letra = match entrada.trim().chars().next(){
            Some(e) => e,
            None => {
                println!("Error solo ingresa una letra");
                continue;
            }
        };

        match letra {
            'A' => {
                entrada.clear();
                println!("ingrese el numero que desea ingredar");
                if let Err(e) = io::stdin().read_line(&mut entrada){
                    eprintln!("error al leer el numero : {}",e);
                } 
                if let Ok(numero) = entrada.trim().parse(){
                    vector_numeros.push(numero);
                } else {
                    println!("No se ingresó un número, inténtalo nuevamente.");
                    continue;
                }
            },
            'B' => {
                entrada.clear();
                println!("ingrese el numero que desea ingredar");
                if let Err(e) = io::stdin().read_line(&mut entrada){
                    eprintln!("error al leer el numero : {}",e);
                } 
                
                if entrada.is_empty() {
                    println!("tiene que ingresar un valor, intentelo nuevamente mas tarde");
                    continue;
                } else {
                    entrada.trim().to_string();
                    vector_mensaje.push(entrada.clone());
                }
            },
            'C' => {
                println!("
                Valores numericos almacenados: {:?}
                mensajes almacenados: {:?}
                ",vector_numeros,vector_mensaje);
            },
            'D' => {
                println!("hasta la proxima.....");
                break;
            }
            _ => {
            println!("Opción no válida, por favor selecciona A, B, C o D.");
            }
            
        };


    }
}