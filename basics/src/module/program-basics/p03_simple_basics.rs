use std::io;

enum EstadoCivil {
    Soltero,
    Pareja,
    Casado,
    Desconocido,
}

// comosi fuera programacion orientada a objetos
impl EstadoCivil {
    // Un método que recibe un texto y devuelve un EstadoCivil
    fn desde_texto(texto: &str) -> Self {
        match texto.trim().to_lowercase().as_str() {
            "soltero" => EstadoCivil::Soltero,
            "pareja" => EstadoCivil::Pareja,
            "casado" => EstadoCivil::Casado,
            _ => EstadoCivil::Desconocido,
        }
    }
}

pub fn estados() {
    let mut entrada = String::new();
    println!("Ingrese su estado (soltero, pareja, casado):");
    io::stdin().read_line(&mut entrada).expect("Error");

    // ¡Mira qué limpio queda ahora!
    let estado = EstadoCivil::desde_texto(&entrada);

    match estado {
        EstadoCivil::Soltero => println!("¡Disfruta tu tiempo libre!"),
        EstadoCivil::Pareja => println!("¡Qué bien! Saludos a tu pareja."),
        EstadoCivil::Casado => println!("¡Mucho éxito en tu matrimonio!"),
        EstadoCivil::Desconocido => println!("No entendí qué estado ingresaste."),
    }
}