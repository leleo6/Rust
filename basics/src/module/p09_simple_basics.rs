

/* 
generiacos
Basicamente son funciones que funcionan con cualquier "tipo de dato"
funciona con funciones, structs, enums, métodos e implementaciones
*/ 

//un genericos implementado en una funcion

fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub fn genericos() {
    let entero = mayor(10, 20);
    let flotante = mayor(3.14, 2.71);
    let caracter = mayor('x', 'a');
    println!("{entero}, {flotante}, {caracter}");
}