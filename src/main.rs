use std::io;

fn main() {
    println!("Adivina el número elegido por Rust");
    println!("--------------------");
    println!("Por favor introduce un número entre el 1 y el 10");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Error al leer el número ingresado");

    println!("El número que elegiste es: {guess}")
}
