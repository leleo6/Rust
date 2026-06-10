use std::sync::Mutex;


pub fn comunicacion_compartida(){
    // Mutex solo un hilo a la vez puede usar la variable
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // adquiere el lock
        *num += 1;                       // modifica el dato
    } // el lock se libera automáticamente
    println!("{:?}", m); // Mutex { data: 6 }
    
}