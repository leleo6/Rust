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
