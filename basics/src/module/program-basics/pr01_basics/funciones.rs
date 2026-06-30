
/*
|||||||funcion||||||||
en rust a las funciones siempre se le debe especificar el tipo
de valor que sale de la variable

Estructura

fn             |crea la funcion| 
n1_hola_mundo  |nombre| 
()             |si nececita recibir argumentos| 
-> ()          |el tipo de salida de la funcion|


*/

fn n1_hola_mundo () -> (){
    println("Hola mundo");
}

/*
Salida

la salida en una funcion de rust tiene dos formas
return;
o
simple mente dejando la salida sin punto y coma al final de la funcion

*/

fn n1_suma (mut num1 : f64, num2 : f64) -> (f64){
    num1 = num1 + num2 
}
/*
funciones (usan referencias)

basicamente son funciones que piden prestado el espacio de memoria de las 
varibles que solician generando un lifetime

osea el la varible donde se almacene el dato que salga de la funcion
solo existira mientras las otras varibles que se solicitaron existan 

*/

fn funcion<'a>(x: &'a mut str) -> &'a str {  
        x
}
