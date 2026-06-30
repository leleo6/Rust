use std::sync::{Arc,Mutex};
use std::thread;


pub fn comunicacion_compartida(){
    // Mutex solo un hilo a la vez puede usar la variable
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // adquiere el lock
        *num += 1;                       // modifica el dato
    } // el lock se libera automáticamente
    println!("{:?}", m); // Mutex { data: 6 }

    // Arch Permite que varios hilos sean dueños compartidos del mismo dato.

    let contador = Arc::new(Mutex::new(0));
    let mut manejadores = vec![];

    for _ in 0..10 {
        let contador = Arc::clone(&contador); // clona el Arc (incrementa el contador)
        let manejador = thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
        });
        manejadores.push(manejador);
    }

    for manejador in manejadores {
        manejador.join().unwrap();
    }

    println!("Resultado final: {}", *contador.lock().unwrap());



    // uso compartido
    let contador = Arc::new(Mutex::new(0));
    let mut manejadores = vec![];

    for _ in 0..10 {
        let contador = Arc::clone(&contador); // clona el Arc (incrementa el contador)
        let manejador = thread::spawn(move || {
            let mut num = contador.lock().unwrap();
            *num += 1;
        });
        manejadores.push(manejador);
    }

    for manejador in manejadores {
        manejador.join().unwrap();
    }

    println!("Resultado final: {}", *contador.lock().unwrap());

}