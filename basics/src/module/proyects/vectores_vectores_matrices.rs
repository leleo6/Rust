pub mod vectores { 
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
}