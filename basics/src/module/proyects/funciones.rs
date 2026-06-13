pub mod funciones_utiles {
    use std::{io,};

        pub struct cedula{
            primer_nombre: String,
            segundo_nombre: String,
            apellidos: String,
            numero_documento: String,
            fecha_nacimiento:String,
        }
        impl cedula {
            pub fn solicitar_datos(& mut self){
                let mut entrada =String::new();

                for i in 1..=5 {
                entrada.clear();
                match i {
                    1 => {
                        self.primer_nombre.clear();
                        println!("Ingresa tu Primer nombre");
                        match io::stdin().read_line(&mut entrada) {
                            Ok(_) => {self.primer_nombre = entrada.trim().to_string()},
                            Err(_) => {panic!()} ,
                        };
                    }
                    2 => {
                        self.segundo_nombre.clear();
                        println!("Ingresa tu segundo nombre");
                        match io::stdin().read_line(&mut entrada) {
                            Ok(_) => {self.segundo_nombre = entrada.trim().to_string()},
                            Err(_) => {panic!()} ,
                    };
                    }
                    3 => {
                        self.apellidos.clear();
                        println!("Ingresa tus apellidos");
                        match io::stdin().read_line(&mut entrada) {
                            Ok(_) => {self.apellidos = entrada.trim().to_string()},
                            Err(_) => {panic!()} ,
                    };
                    }
                    4 => {
                        self.numero_documento.clear();
                        println!("Ingresa tu numero de documento nombre");
                        match io::stdin().read_line(&mut entrada) {
                            Ok(_) => {self.numero_documento = entrada.trim().to_string()},
                            Err(_) => {panic!()} ,
                    };
                    }
                    5 => {
                        self.fecha_nacimiento.clear();
                        println!("Ingresa tu fecla de nacimiento nombre");
                        match io::stdin().read_line(&mut entrada) {
                            Ok(_) => {self.fecha_nacimiento = entrada.trim().to_string()},
                            Err(_) => {panic!()} ,
                    };
                    }

                    _ => {}
                    
                };
            };
        }
    }
}