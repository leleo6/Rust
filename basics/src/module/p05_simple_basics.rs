//manejo de errores

pub fn errores(){
    //useo de match para errores cuando se nececita hacer acciones complejas cuanco aparece un error 
        match io::stdin().read_line(&mut entrada_menu) {
            Ok(_) => {},
            Err(_) => {entrada_menu = "error al recibir la entrada del menu".to_string()} ,
        }
    // if let se usa cuando no importa el caso OK()
        loop {
            let mut entrada_menu : String = String::new();
            println!("!!!!Bienvenido A cotizador ADSO!!!!");
            menu_completo();
            if let Err(e) =  io::stdin().read_line(&mut entrada_menu){
                eprintln!("error al recibir la entrada del menu");
                continue;//hace otra iteracion del loop como un "intentalo de nuevo"
            }

        }
    
}