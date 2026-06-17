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



Referencia
basicamente solo se usa cuando se nececitan comparar datos y guardar datos de forma temporal (es practicamente lo mismo que &variable)
// 'a anota el lifetime de LAS REFERENCIAS, no de la función en sí

    fn funcion<'a>(x: &'a str) -> &'a str {  // & = referencia
        x
    }

    // 'a anota el lifetime de LAS REFERENCIAS en el closure
    let closure = |x: &'a str| -> &'a str {  // & = referencia
        x
    };

    // 'a anota el lifetime de LAS REFERENCIAS en el struct
    struct Estructura<'a> {
        campo: &'a str,  // & = referencia
    }