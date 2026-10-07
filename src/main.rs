use std::io::stdin;

const SALIR: &str = "salir";
fn main() {
    let mut palabra = String::new();
    let mut palabra_deletreada = String::new();
    let mut continuar: bool = true;
    println!("Deletrear palabra");
    println!("-----------------");
    println!("\n");
    while continuar {
        println!("Escriba la palabra:");
        palabra.clear();
        stdin().read_line(&mut palabra).expect("Error leyendo la palabra");
        let palabra_limpia = palabra.trim();
        if palabra_limpia == SALIR {
            println!("Adiós...");
            continuar = false
        }
        else {
            for letra in palabra_limpia.chars() {
            palabra_deletreada.push_str(&format!("{}-", letra));
        }
        }

        println!("{palabra_deletreada}");
        palabra_deletreada.clear();
        if palabra_limpia == SALIR {continuar = false};
    }
}

