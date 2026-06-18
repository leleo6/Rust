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


Iteradores
creacion de iteradores usa estos metodos
 - .iter      |toma los elementos como una referencia inmutable
 - .iter_mut  |toma los elementos como una referencia mutable
 - .into_iter |toma la propiedad total de los elementos que maneja

NOTA: Cuando escribes for x in numeros (sin llamar a ningún método), Rust automáticamente expande eso a for x in numeros.into_iter().

mas metodos

.map()        | aplica una funcion(closures) en cada iteracion
.filter()     | aplica una condicion(true or false) para filtar se usa con closures
.filter_map() | debuelve un tipo option (some(guarda el valor), none(elimina el valor))
.enumerate    | Modifica el iterador para que devuelva una tupla

por ultimo para recoger lo optenido de los iteradores se usa
.collect()    | Transforma el iterador de vuelta en una colección (como un Vec, HashMap, etc.). 
                Rust es tan inteligente que infiere el tipo de colección que quieres si se lo especificas en la variable.