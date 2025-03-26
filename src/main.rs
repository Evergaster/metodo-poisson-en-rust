use std::f64::consts::E;
use std::io::{self, Write};

struct poisson {
    lambda: f64,
}

impl poisson {
    fn funcion(&self, n: u32) -> f64 {
        let numerator = self.lambda.powi(n as i32) * E.powi(-self.lambda as i32);
        let mut factorial = 1.0;
        for x in 1..=n {
            factorial *= x as f64;
        }
        numerator / factorial
    }
}

fn main() {
    let mut lambda = 0.0;
    let mut n = 0;
    let mut input = String::new();

    loop {
        println!("1. agrega el valor de lambda");
        println!("2. agrega el valor de x");
        println!("3. mostrar resultado de la funcion de poisson");
        println!("4. mostrar formula");
        println!("5. salir");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let opcion: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match opcion {
            1 => {
                println!("Agrega el valor de lambda");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                lambda = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Error al agregar lambda");
                        continue;
                    }
                };
                println!("lambda se ha agregado correctamente");
            }
            2 => {
                println!("Agrega el valor de x");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                n = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Error al agregar x");
                        continue;
                    }
                };
                println!("n se ha agregado correctamente");
            }
            3 => {
                if lambda == 0.0 || n == 0 {
                    println!("Agrega el valor de lambda y n");
                    return;
                } 
                    let poisson = poisson { lambda: lambda };
                    let result = poisson.funcion(n);
                    println!("El resultado de la funcion de poisson es: {}", result);
                    println!("En porcentaje: {}%", result * 100.0);
                
            }
            4 => {
                println!("Poisson Distribution Formula");
                println!("P({};{}) = ({}^{} * {}^(-{})) / {}!", n, lambda, lambda, n, E, lambda, n);
            }
            5 => break,
            _ => {
                println!("Opcion no valida");
            }
        }
    }
}