use std::fmt::Write;

fn main() {
    println!("--- 1. println! con literal ---");
    imprimir_con_literal();

    println!("\n--- 2. String + format! ---");
    imprimir_con_format();

    println!("\n--- 3. to_string() ---");
    imprimir_con_to_string();

    println!("\n--- 4. concat! ---");
    imprimir_con_concat();

    println!("\n--- 5. format_args! + write! ---");
    imprimir_con_format_args();

    println!("\n--- 6. String + &str con + ---");
    imprimir_con_suma();

    println!("\n--- 7. push_str ---");
    imprimir_con_push_str();

    println!("\n--- 8. join() ---");
    imprimir_con_join();
}

fn imprimir_con_literal() {
    println!("hola mundo - {}", std::any::type_name::<&str>());
}

fn imprimir_con_format() {
    let saludo = String::from("hola mundo");
    let tipo = std::any::type_name::<String>();
    let mensaje = format!("{} - {}", saludo, tipo);
    println!("{}", mensaje);
}

fn imprimir_con_to_string() {
    let saludo = "hola mundo".to_string();
    println!("{} - {}", saludo, std::any::type_name::<String>());
}

fn imprimir_con_concat() {
    // Solo funciona con literales en tiempo de compilación
    const MENSAJE: &str = concat!("hola mundo", " - ", "static");
    println!("{}", MENSAJE);
}

fn imprimir_con_format_args() {
    let mut buffer = String::new();
    write!(&mut buffer, "hola mundo - {}", std::any::type_name::<String>()).unwrap();
    println!("{}", buffer);
}

fn imprimir_con_suma() {
    let saludo = "hola mundo".to_string();
    let tipo = " - String";
    let mensaje = saludo + tipo;
    println!("{}", mensaje);
}

fn imprimir_con_push_str() {
    let mut mensaje = String::from("hola mundo");
    mensaje.push_str(" - ");
    mensaje.push_str(std::any::type_name::<String>());
    println!("{}", mensaje);
}

fn imprimir_con_join() {
    let partes = vec!["hola mundo", std::any::type_name::<&str>()];
    let mensaje = partes.join(" - ");
    println!("{}", mensaje);
}
