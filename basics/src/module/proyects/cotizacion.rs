use std::{io};

pub fn cotizacion_estudiantes(){
    let mut precio_comida = String::new();
    let mut cantidad_estudiantes = String::new();

    println!("Ingresa la cantidad de estudiantes");
    // solicitamos la cantidad de estudiantes que van a participar 
    io::stdin()
        .read_line(&mut cantidad_estudiantes)
        .expect("error al leerlo");
    // solicitamos el precio de la comida que se va a comprar
    println!("Ingrese el precio de la comida que se quiere comprar: ");
    io::stdin().read_line( &mut precio_comida).expect("error al leerlo");
    println!();
    // se muestran los datos solicitados por los usuarios
    print!("cantidad ingresada: {cantidad_estudiantes}");
    println!("precio ingresado: {precio_comida}");

    let PRECIO : f64 = match precio_comida.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("error en la lectura");
            return;
        }
    };
    let CANTIDAD : f64 = match cantidad_estudiantes.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("error en la lectura");
            return;
        }
    };

    let mut precio_final:f64 = PRECIO*CANTIDAD;

    println!("el valor a pagar es: {precio_final}");
    
}