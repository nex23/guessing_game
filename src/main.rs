use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el número elegido por Rust");

    let secret_number = rand::thread_rng().gen_range(1..=50);

    println!("--------------------");
    println!("Por favor introduce un número entre el 1 y el 50");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Error al leer el número ingresado");

    println!("El número que elegiste es: {guess}");

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Muy pequeño"),
      Ordering::Greater => println!("Muy grande"),
      Ordering::Equal => println!("Adivinaste!"),
    }
}
