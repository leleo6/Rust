fn funciones(mut baisco:i32, caratacter:char) ->i32{ //para que la funcion tenga un retorno siempre hay que especificar el tipo
    print!("{}",caratacter);
    baisco = 1;
    baisco + 12  // el valor a retornar no debe tener ; 
}

pub fn basic_main(){
    //Type variables
    let _num1:i32=22;    // numeros enteros todos los que comiencen con i
    let _num2:f32=2.3;   // numeros flotantes o condecimales comienzan con f
    let _word1:&str="loco";// cadena de caracteres 
    let _word2:char='c'; // un caracter tambien acepta emogis
    // toda variable que no tenga mut es una constante
    let mut _num3:i32=1; //mut deja que el valor de la variable se pueda cambiar

    //Arrys y tuplas
    let _numbers:[i32;4] =[1,2,3,532]; // arry
    let _usuarios:(&str,i32,f32) = ("sasas",12,2.4); // tupla

    //condicionales
    let cont:i32 = 0;

    if cont <=0 {
        println!("numero es menor o igual a 0")
    }
    else if cont>0 {
        println!("numero mayor a 0");
    }else {
        print!("error desconocido");
    }

    //ciclos
    for i in 1..=5{
        //del 5 ciclos
    }
    while _num1 == 0 {
        print!("numero 0");
    }
    loop { // mantiene el ciclo hasta que encuentra un break
        _num3 += 1;
        if  _num3 != 0 {
            break;
        }
    }


    println!("hola mundo");

} 