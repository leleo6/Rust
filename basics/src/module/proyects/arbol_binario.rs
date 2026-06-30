


// 1. Definimos el Nodo del árbol
#[derive(Debug)]
struct Nodo {
    valor: i32,
    izq: Option<Box<Nodo>>,
    der: Option<Box<Nodo>>,
}

impl Nodo {
    fn nuevo(valor: i32) -> Self {
        Nodo {
            valor,
            izq: None,
            der: None,
        }
    }
}

// 2. Definimos la estructura principal del Árbol
#[derive(Debug)]
pub struct ArbolBinario {
    raiz: Option<Box<Nodo>>,
}

impl ArbolBinario {
    // Crea un árbol vacío
    pub fn nuevo() -> Self {
        ArbolBinario { raiz: None }
    }
    // RETO 1: Insertar un valor en el árbol respetando las reglas de un BST:
    // - Si el valor es menor que el nodo actual, va a la izquierda.
    // - Si es mayor, va a la derecha.
    // - Si es igual, no hacemos nada (evitamos duplicados).
    pub fn insertar(&mut self, valor: i32) {
        Self::insertar_recursivo(&mut self.raiz, valor);
    }

    // Función auxiliar recursiva para insertar
    fn insertar_recursivo(nodo_opt: &mut Option<Box<Nodo>>, valor: i32) {
        match nodo_opt {
            Some(_) => {}
            None => {}
        }


        // ¡TU CÓDIGO AQUÍ!
        // Pista: Usa match sobre `nodo_opt` con `ref mut` para poder modificar los hijos.
        // Si el match es `None`, crea un nuevo Nodo metido en un Box usando `Some(Box::new(Nodo::nuevo(valor)))`.
        // Si es `Some(nodo)`, decide si ir por la izquierda o derecha recursivamente.
    }

    // RETO 2: Buscar si un valor existe en el árbol.
    // Debe devolver `true` si existe, o `false` en caso contrario.
    pub fn buscar(&self, valor: i32) -> bool {
        Self::buscar_recursivo(&self.raiz, valor)
    }

    // Función auxiliar recursiva para buscar
    fn buscar_recursivo(nodo_opt: &Option<Box<Nodo>>, valor: i32) -> bool {
        // ¡TU CÓDIGO AQUÍ!
        // Pista: Usa match sobre `nodo_opt`.
        // Si es `None`, retorna `false`.
        // Si es `Some(nodo)`, compara el valor con `nodo.valor`:
        // - Si son iguales, retorna `true`.
        // - Si es menor, busca recursivamente en la izquierda.
        // - Si es mayor, busca recursivamente en la derecha.
        false // Quitar esto cuando lo completes
    }
}

// Función para probar tu implementación
fn main() {
    let mut arbol = ArbolBinario::nuevo();
    
    // Insertamos datos
    arbol.insertar(15);
    arbol.insertar(10);
    arbol.insertar(20);
    arbol.insertar(5);
    arbol.insertar(12);

    println!("Estructura del árbol: {:#?}", arbol);

    // Probamos la búsqueda
    println!("¿Contiene el 12? {}", arbol.buscar(12)); // Debería ser true
    println!("¿Contiene el 8? {}", arbol.buscar(8));   // Debería ser false
}