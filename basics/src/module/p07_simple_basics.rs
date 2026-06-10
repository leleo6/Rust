use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn concurrencia(){
    let manejador = thread::spawn(|| {
        for i in 1..=5 {
            println!("Hilo secundario: iteración {i}");
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Hilo principal hace lo suyo
    for i in 1..=3 {
        println!("Hilo principal: iteración {i}");
        thread::sleep(Duration::from_millis(70));
    }

    // Esperar a que el hilo secundario termine
    manejador.join().unwrap(); // unwrap maneja un posible panic en el hilo

    let mensaje = String::from("Hola desde el hilo");
    let manejador = thread::spawn(move || {
        println!("{mensaje}");
    });
    // println!("{mensaje}"); // Error: mensaje fue movido al hilo
    manejador.join().unwrap();

    let (tx, rx) = mpsc::channel(); // tx: transmisor, rx: receptor

    thread::spawn(move || {
        tx.send("hola").unwrap();      // envía un mensaje
        tx.send("mundo").unwrap();
        // tx sale de ámbito aquí, el canal se cierra
    });

    // El receptor itera hasta que el canal se cierra
    for mensaje in rx {
        println!("Recibido: {mensaje}");
    }

}