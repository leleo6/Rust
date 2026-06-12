pub mod funciones_utiles {
    use std::{io, option};

    struct cedula{
        primer_nombre: String,
        segundo_nombre: String,
        apellidos: String,
        numero_documento: String,
        fecha_nacimiento:String,
    }
    impl cedula {
        fn solicitar_datos(&self) -> cedula{
            let datos = cedula{
                primer_nombre:   "32".to_string(),
                segundo_nombre:  "32".to_string(),
                apellidos:       "32".to_string(),
                numero_documento:"32".to_string(),
                fecha_nacimiento:"32".to_string(),
            };
            let mut entrada =String::new();
            for i in 1..5 {
              entrada.clear();
              match i {
                1 => {
                    datos.primer_nombre.clear();
                    println!("Ingresa tu Primer nombre");
                    match io::stdin().read_line(&mut entrada) {
                        Ok(_) => {entrada.trim().to_string()},
                        Err(_) => {panic!()} ,
                    };
                    

                }
                  
              }  
            };
        }
        
    }
}