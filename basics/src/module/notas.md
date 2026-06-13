Option<T>
some(t) // basicamente si hay algo lo guarda en la variable t 
none // y si no hay nada no hace nada o hace otra cosa

    let edad: Option<i32> = Some(25);

    match edad {
        Some(e) => println!("Tienes {} años.", e),
        None => println!("No especificaste tu edad."),
    }

Result<T, E>
Ok(T)
Err(E)
// basicamente es lo mismo si esta bien lo lleva a T y si esta mal a E



todo es privado al 100% hasta que se le pone el pub (aunque la funcion este adentro de un mod pub es privada hasta que se le ponga el pub)