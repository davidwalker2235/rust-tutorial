fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {

    println!("Convertir tipo de datos &str a String:\n");

    let data_str: &str = "Hola mundo";
    println!("el tipo de datos de data_str es: {:?}", type_of(&data_str));
    let data_string = data_str.to_string();
    println!("el tipo de datos de data_str es: {:?}", type_of(&data_string));

    println!("\nConvertir tipo de datos String a &str:\n");
    let string = "hola que tal".to_owned();
    println!("el tipo de datos de string es: {:?}", type_of(&string));
    let str_chad = string.as_str();
    println!("ahora el tipo de datos de string es: {:?}", type_of(&str_chad));

    println!("\nConvertir tipo de datos String a int:\n");
    let number = "100".to_owned();
    println!("El tipo de datos de number es: {:?}", type_of(&number));
    let conversion: i32 = number.parse().expect("no es un INT");
    println!("ahora el tipo de datos de number es: {:?}", type_of(&conversion));

    println!("\nConvertir tipo float a int:\n");
    let float: f64 = 100.5;
    println!("El tipo de datos de float es: {:?}", type_of(&float));
    let conversion_int: i32 = float.round() as i32;
    println!("ahora el tipo de datos de float es: {:?}", type_of(&conversion_int));
    println!("el float es: {:?}", &conversion_int);
}