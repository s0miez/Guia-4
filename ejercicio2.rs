use std::io;
fn arreglo() { //insertar esta funcion en fn main.
    let a: [u8; 10] = [num1, num2, num3, num4, num5, num6, num7, num8, num9, num10];
    let sum: u8 = a.iter().sum();
}


fn ingreso() { // intentar usar ciclo para los inputs.
    let mut num1:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num1).unwrap();
    let num1: u8 = num1.trim().parse().unwrap();

    let mut num2:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num2).unwrap();
    let num2: u8 = num2.trim().parse().unwrap();

    let mut num3:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num3).unwrap();
    let num3: u8 = num3.trim().parse().unwrap();

    let mut num4:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num4).unwrap();
    let num4: u8 = num4.trim().parse().unwrap();

    let mut num5:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num5).unwrap();
    let num5: u8 = num5.trim().parse().unwrap();

    let mut num6:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num6).unwrap();
    let num6: u8 = num6.trim().parse().unwrap();

    let mut num7:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num7).unwrap();
    let num7: u8 = num7.trim().parse().unwrap();

    let mut num8:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num8).unwrap();
    let num8: u8 = num8.trim().parse().unwrap();

    let mut num9:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num9).unwrap();
    let num9: u8 = num9.trim().parse().unwrap();

    let mut num10:String = String::new();
    println!("Ingrese un número: ");
    let stdin = io::stdin();
    stdin.read_line(&mut num10).unwrap();
    let num10: u8 = num10.trim().parse().unwrap();
}
fn main() {

}

// termimnar...
