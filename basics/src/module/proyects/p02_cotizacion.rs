use std::io;
#[allow(dead_code)]
struct PosiblesComidas{
    nombre: String,
    precio: f64,
    porciones:i32,
}
#[allow(dead_code)]
pub fn cotizacion_nivel2(){

    let mut nombre_01 = String::new();
    let mut precio_01 = String::new();
    let mut porciones_01 = String::new();
    let mut cantidad_estudiantes = String::new();

    // solicitamos los datos que nececitamos

    println!("Ingrese el nombre del producto: ");
    io::stdin().read_line( &mut nombre_01).expect("error de lectura intentalo mas tarde");
    println!("Ingrese el precio del producto: ");
    io::stdin().read_line( &mut precio_01).expect("error de lectura intentalomas tarde");
    println!("Ingrese las porciones que lleva cada producto: ");
    io::stdin().read_line(&mut porciones_01).expect("error de lectura intentalo mas tarde");
    println!("Ingrese la cantidad de estudiantes: ");
    io::stdin().read_line(&mut cantidad_estudiantes).expect("error de lectura intentalo mas tarde");

    // combertimos y limpiamos los datos los mandamos al struct

    let _producto_final = PosiblesComidas{
        nombre: nombre_01.trim().to_string(),
        precio: precio_01.trim().parse().expect("precio invalido"),
        porciones: porciones_01.trim().parse().expect("invalido")
    };
    let _cantidad_estudiantes_final: i32= cantidad_estudiantes.trim().parse().expect("cantidad de estudiantes invalida"); 

    //calculamos los resultados
    

    
 




}