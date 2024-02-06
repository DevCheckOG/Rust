mod util;

fn main() {
    loop {

        let vec_of_valid_values : Vec<usize> = vec![1, 2, 3, 4, 5];
        
        print!("\nCalculadora en RUST\n\n");
        
        print!("Ingresa la tipo de operacion: 
        
    1. Suma
    2. Resta
    3. Multiplicación
    4. Division
    5. Salir
        
        \n");

        let mut value: String = String::new();
        let mut value_one: String = String::new();
        let mut value_two: String = String::new();

        std::io::stdin().read_line(&mut value).expect("\nError al leer la entrada del usuario.\n");
        
        let option: Option<usize> = match value.trim().parse() {

            Ok(value) => Some(value),
            Err(_) => {

                print!("\n---------------------------------");
                print!("\nValores válidos: 1, 2, 3, 4, 5\n");
                print!("---------------------------------\n");
                continue;

            }

        };

        if !vec_of_valid_values.contains(&option.unwrap()) {
            print!("\n---------------------------------");
            print!("\nValores válidos: 1, 2, 3, 4, 5\n");
            print!("---------------------------------\n");
            continue;
        } else if option == Some(5) {
            break;
        }

        print!("\nIngresa el primer valor: \n");
        std::io::stdin().read_line(&mut value_one).expect("\nError al leer la entrada del usuario.\n");

        let num_one: Option<usize> = match value_one.trim().parse() {

            Ok(value) => Some(value),
            Err(_) => {

                print!("\n---------------------------------");
                print!("\n{} es un valor inválido.\n", value.trim());
                print!("---------------------------------\n");
                continue;

            }
        };

        print!("\nIngresa el segundo valor: \n");
        std::io::stdin().read_line(&mut value_two).expect("Error al leer la entrada del usuario.");

        let num_two: Option<usize> = match value_two.trim().parse() {

            Ok(value) => Some(value),
            Err(_) => {

                print!("\n---------------------------------");
                print!("\n{} es un valor inválido.\n", value.trim());
                print!("---------------------------------\n");
                continue;

            }
        }; 

        match &option {

            Some(1) => {
                print!("\n---------------------------------");
                print!("\n{} + {} = {}\n", num_one.unwrap(), num_two.unwrap(), util::sum(num_one.unwrap(), num_two.unwrap()));
                print!("---------------------------------\n");        
            },
            Some(2) => {
                print!("\n---------------------------------");
                print!("\n{} - {} = {}\n", num_one.unwrap(), num_two.unwrap(), util::rest(num_one.unwrap(), num_two.unwrap()));
                print!("---------------------------------\n");
            },
            Some(3) => {
                print!("\n---------------------------------");
                print!("\n{} * {} = {}\n", num_one.unwrap(), num_two.unwrap(), util::mult(num_one.unwrap(), num_two.unwrap()));
                print!("---------------------------------\n");
            },
            Some(4) => { 

                match util::div(num_one.unwrap(), num_two.unwrap()) {

                    (0, false) => {

                        print!("\n---------------------------------");
                        print!("\nNo se puede dividir entre 0.\n");
                        print!("---------------------------------\n");
                        continue;

                    },
                    _ => {
                        print!("\n---------------------------------");
                        print!("\n{} / {} = {}\n", num_one.unwrap(), num_two.unwrap(), util::div(num_one.unwrap(), num_two.unwrap()).0);
                        print!("---------------------------------\n");
                    },
                    
                }
        
            },

            _ => {
                print!("\n---------------------------------");
                print!("\n{} es un valor inválido.\n", option.unwrap());
                print!("---------------------------------\n");
                continue;
            }
            
        }

    }
}
