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