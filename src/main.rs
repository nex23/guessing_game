use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el número elegido por Rust");

    let secret_number = rand::thread_rng().gen_range(1..=50);
    loop {
      println!("--------------------");
      println!("Por favor introduce un número entre el 1 y el 50");

      let mut guess = String::new();

      io::stdin()
        .read_line(&mut guess)
        .expect("Error al leer el número ingresado");

        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };

      println!("El número que elegiste es: {guess}");

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Muy pequeño, intenta de nuevo"),
        Ordering::Greater => println!("Muy grande, intenta de nuevo"),
        Ordering::Equal => {
          println!("Sos un capo. ADIVINASTE!");
          break;
        }
      }
    }
}
