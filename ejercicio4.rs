fn main() {
    let mut arreglo1: [u8; 5] = [1, 2, 3, 4, 5];
    let mut arreglo2: [u8; 5] = [6, 7, 8, 9, 10];
    let mut resultado = Vec::with_capacity(arreglo1.len());

    for i in 0..arreglo1.len() {
        resultado.push(arreglo1[i] * arreglo2[i]);
    println!("La multiplicación de los arreglos es: {:?}", resultado);
    }
}
