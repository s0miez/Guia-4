use std::io;
fn main() {
    let mut numero:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut numero).unwrap();
    let numero: u8 = numero.trim().parse().unwrap();

    let mut arreglo = [0, 1, 2, 3, 4]; 

    let numero = match arreglo {
        0 => println!("Dentro del arreglo!, Cero"),
        1 => println!("Dentro del arreglo!, Uno"),
        2 => println!("Dentro del arreglo!, Dos"),
        3 => println!("Dentro del arreglo!, Tres"),
        4 => println!("Dentro del arreglo!, Cuatro"),
        _ => println!("Fuera del arreglo :("),
   };
}
