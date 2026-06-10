//Closures basicamente mini-funciones si muladas como una variable
pub fn simple_basic(){
    
    let  suma_enteros = |x:i32|{x+1};
    let  num1 = suma_enteros(6);
    //entonces num1 imprimiria un 7
    println!("6 + 1= {}",num1);

    // Captura del entorno
    let limite = 10;
    let es_mayor_que_limite = |x: i32| x > limite; // captura &limite inmutable
    // la bariable de arriba devuelve un boleano true or falce

    for n in [5, 12, 8] {
        println!("{n} > {limite}? {}", es_mayor_que_limite(n));
    }
    //si es mayor imprime true y si es menor imprime false

    // Captura mutable
    let mut contador = 0;
    let mut incrementar = || {
        contador += 1;          // captura &mut contador
        contador
    };

    println!("Incremento: {}", incrementar());
    println!("Incremento: {}", incrementar());

    //cada vez que se usa la funcion anonima incrementa la variable contador

}

/*
Iteradores
Basicamente es como una sinta transportadora en una fabrica
en ella se hacen diversas cosas como sellar o empacar 
antes de llegar a un producto final y ese seria el .collect()

y esto basicamente se hace usando metodos
(aclaracion para que sea una iteracion tiene que recorrer algo como un vector
o hacer un ciclo que de multiples vueltas)


*/
fn simple_basic_02(){
    let numeros = vec![1, 2, 3, 4, 5];
    let cuadrados_pares: Vec<i32> = numeros
        .iter()               // iterador sobre referencias &i32
        .filter(|&&x| x % 2 == 0) // filtramos pares
        .map(|&x| x * x)       // elevamos al cuadrado
        .collect();             // recolectamos en Vec
    println!("{:?}", cuadrados_pares); // [4, 16]
}