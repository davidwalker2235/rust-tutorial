use std::env::args;

fn main() {
let mut args: Vec<String> = args().collect();

args.remove(0);

for i in &args {
    if i.len() > 2 {
        let parametro = &i[..2];
        let valor = &i[3..];
        match parametro {
            "-t" => println!("Argumento Time: {:?}, {:?}", parametro, valor),
            "-o" => println!("Argumento Time: {:?}, {:?}", parametro, valor),
            "-i" => funcion_i(),
             _ => println!("[ERROR], no existe el parámetro"),
        }
    }
    else {
        if i.contains("-h") {
            println!("Ayuda de parámetros");
            println!("-t:[numero entero]");
            println!("-o:[output]");
            println!("-i:[input]");
        }
        else {
            println!("Error en parámetro. Escriba -h para ayuda");
        }
    }

    fn funcion_i () {
        println!("se ejecula la función i");
    }
}
    

}