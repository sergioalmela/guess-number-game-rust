use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    // Generamos un número entre el 1 y el 100
    let random_number = rng.gen_range(1..101);

    // Hasta que no acertamos el número no salimos del bucle
    loop {
        println!("Introduce un número entre el 1 y el 100");

        // Requerimos al usuario que introduzca un número, para posteriormente convertirlo a integer
        let mut reader = String::new();
        io::stdin()
        .read_line(&mut reader)
        .expect("Fallo al leer stdin");
        
        let input_text: i32 = reader.trim().parse().expect("Debes de un introducir un número válido!");

        // Hacemos las comprobaciones
        if input_text > random_number {
            println!("El número introducido es mayor, vuelve a intentarlo");
        } else if input_text < random_number {
            println!("El número introducido es menor, vuelve a intentarlo");
        } else {
            println!("El número introducido es CORRECTO!!");
            break;
        }
    }
}
