/*
||||||||||TEMA FINAL BASICS ||||||||||||
trait
define un conjunto de métodos que un tipo debe implementar para cumplir el contrato.
Luego puedes escribir código que acepte cualquier tipo que tenga ese certificado (trait bound).

NOTA:despues de esto ya solo se repasara lo mismo desde
diferentes puntos de vista o se agregaran librerias 
aqui termina lo basico y comienza el intermedio
*/

trait Saludable {
    fn estado(&self) -> String;
    fn curar(&mut self); // sin implementación por defecto
}

struct Guerrero {
    nombre: String,
    vida: u32,
}

impl Saludable for Guerrero {
    fn estado(&self) -> String {
        format!("{}: {} HP", self.nombre, self.vida)
    }
    fn curar(&mut self) {
        self.vida = 100;
    }
}

fn main() {
    let mut g = Guerrero { nombre: "Aragorn".into(), vida: 45 };
    println!("{}", g.estado());
    g.curar();
    println!("{}", g.estado());
}