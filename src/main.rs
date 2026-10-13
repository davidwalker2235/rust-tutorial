use std::{io::{self, stdin}, ops::ControlFlow::Break};

use os_info::Type::Mabox;

struct Calculo {
    num_a: f32,
    num_b: f32,
}

impl Calculo {
    fn entrada(&mut self, a:f32, b:f32) {
        self.num_a = a;
        self.num_b = b;
    }

    fn suma(&self) -> f32{
        return self.num_a + self.num_b;
    }

    fn resta(&self) -> f32{
        return self.num_a - self.num_b;
    }

    fn mostrar(&self) {
        println!("El número a es: {} y el número b es: {}", self.num_a, self.num_b);
    }
}



fn main() {

    let operacion: Calculo = Calculo { num_a: 0.0, num_b: 0.0 };

    loop {
        println!("------------------------------");
        println!("Opciones:");
        println!("1 Entrada de datos");
        println!("2 Suma");
        println!("3 Resta");
        println!("4 Mostrar datos");
        println!("5: Salida");
        println!("------------------------------");


        let mut entrada: String = String::new();
        io::stdin().read_line(&mut entrada).expect("fallo de entrada");
        let selection: i8;

        match entrada.trim().parse::<i8>() {
            Ok(n) => {selection = n},
            Err(_) => {selection = 0},
        }

        match selection {
            1 => {
                let mut valor_1: String = String::new();
                let mut valor_2: String = String::new();
                println!("Asignar valor 1:");
                io::stdin().read_line(&mut valor_1).expect("error en entrada valor 1");

                println!("Asignar valor 2:");
                io::stdin().read_line(&mut valor_2).expect("error en entrada valor 2");

                println!("Los nuevos valores son: Valor 1 = {}, valor 2 = {}", &valor_1, &valor_2);
            },
            5 => {
                break;
            }

            _ => {
                break;
            }
        }
    };
}

