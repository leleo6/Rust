#[allow(dead_code)]

pub mod vectores {
    use core::num;
 
    fn p100_llenar_vector_limite_i32(mut vector_para:Vec<i32>) -> Vec<i32>{
        //i32::Max representa el numero maximo que puede tener un tipo i32


        //esta funcion posiblemente crasee por la cantidad tan masiva de datos que almacena en un vector
        for i in 0..i32::MAX {
            vector_para.push(i);
        };
        vector_para

    }

    // esta si funciona por que no los almacena solo los muestra
    
    fn postrar_minimo_de_diferentes_tipos(){
        for i in 0..i32::MIN{
            println!("{}",i)
        }
    }

    fn limpiar_eliminar_indices(){
        let mut numeros = vec![1, 2, 4, 5, 6, 7];

        // Elimina el elemento en ese índice y desplaza todos los siguientes a la izquierda
        numeros.remove(0); // Queda: [2, 4, 5, 6, 7]
        println!("Luego de remove(0): {:?}", numeros);

        // Toma el último elemento y lo intercambia por el del índice indicado
        numeros.swap_remove(1); // El 4 (índice 1) se elimina, el 7 (último) pasa a su lugar.
        println!("Luego de swap_remove(1): {:?}", numeros); // Queda: [2, 7, 5, 6]

        numeros.push(0); // Pone el cero al final -> [2, 7, 5, 6, 0]

        numeros.insert(0, 2); // Inserta el 2 al inicio (índice 0). Sacrifica rendimiento. -> [2, 2, 7, 5, 6, 0]

        numeros.push(2); // Agrega el número al final del vector -> [2, 2, 7, 5, 6, 0, 2]
        println!("Resultado final vectores: {:?}", numeros);
    }

    fn matrices(){
        // Al agregar "mut", ahora sí podemos cambiar los valores internos de las celdas
        // Pero su tamaño sigue siendo estrictamente fijo (3x4)
        let mut matriz: [[i32; 4]; 3] = [ 
            [1, 2, 3, 4],    // Fila 0
            [5, 6, 7, 8],    // Fila 1
            [9, 10, 11, 12]  // Fila 2
        ];

        matriz[0][0] = 99; // ¡Esto es perfectamente válido gracias al `mut`!

        // Matriz dinámica (Vector de vectores)
        let mut matriz_dinamica: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![4, 5, 6]
        ];
        
        // Puedes añadir una fila completa dinámicamente aumentando la memoria
        matriz_dinamica.push(vec![7, 8, 9]);
        println!("Matriz dinámica final: {:?}", matriz_dinamica);
    }
}