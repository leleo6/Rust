use core::num;
use std::io;

use std::collections;

pub fn manejo_datos(){
    let mut numeros:Vec<i32> =Vec::new(); //vector vacio
    //llenamos el vector
    numeros.push(10);
    numeros.push(20);
    numeros.push(30);
    //imprimimos 
    println!("los numeros que hay en en vector son {:?}",numeros);
    println!("los numeros que hay en en vector son {}",numeros[0]);

    let mut precios = vec![1.99, 2.49, 0.99]; // macro vec!
    precios.pop(); // elimina y devuelve el último (Some(0.99))
    println!("Segundo elemento: {}", precios[1]);

    // Iterar con for
    for precio in &precios {
        println!("Precio: {precio}");
    }



    

}