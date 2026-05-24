struct comida {
    nombre: string,
    precio : f32,
    tipo: string
}


pub fn estructuras(){
    let COM1 = comida {
        nombre: "pizza",
        precio: 15.000,
        tipo: "solida"
    };
    println("{}", COM1.nombre);
}