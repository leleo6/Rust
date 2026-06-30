use std::io;

struct rectangulo {
    ancho: u32,
    alto: u32,
}
// mismo nombre que el del struct
impl rectangulo{
    // funcion asociada
    fn area (&self) -> u32{
        self.ancho * self.alto
    }
    // uso de funcion adentro de funcion
    fn puede_contener(&self,otro:&rectangulo) -> bool{
        if self.area() == otro.area(){println!("rectangulos iguales")};
        self.area() > otro.area()
    }
    // uso de un struct dentro de su funcion asociada
    fn cuadrado(lado:u32) ->rectangulo{
        rectangulo{
            ancho:lado,
            alto:lado,
        }
    }
}

pub fn estructuras(){

    loop {
    let mut entrada_ancho =String::new();
    let mut entrada_alto =String::new();

    // se solicitan y limpian los datos al usuario

    println!("RECTANGULOS");
    println!("Ingrese el ancho del rectangulo: ");
    io::stdin().read_line(&mut entrada_ancho).expect("error al leer el input");
    let ancho1 : u32 = entrada_ancho.trim().parse().expect("error al trasformar el dato");
    println!("Ingrese el alto del rectangulo: ");
    io::stdin().read_line(&mut entrada_alto).expect("error en la leida");
    let alto1: u32 =entrada_alto.trim().parse().expect("error en trasnformar el dato");
    
    // llenamos el struct
    let num1 = rectangulo{
        ancho: ancho1,
        alto: alto1
    };
    // usamos la funcion asociada
    println!("el area es {} ",num1.area());
        
    }


}