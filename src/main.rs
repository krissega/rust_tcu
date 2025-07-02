fn main() {
    propiedad();
    referencias();
    referencias_mutables();
    una_referencia_mutable();
    mutable_y_no_mutable();
    clonacion();
    tipos_copy();
}

fn propiedad() {
    println!("--- Propiedad (Ownership) ---");
    let nombre = String::from("Sebas");
    saludar_propiedad(nombre);
    // println!("{}", nombre); // ❌ Error: nombre ya no es válido aquí
}

fn saludar_propiedad(texto: String) {
    println!("Hola, {}", texto);
}

fn referencias() {
    println!("\n--- Referencias (Borrowing) ---");
    let nombre = String::from("Mundo");
    saludar_referencia(&nombre);
    println!("Después: {}", nombre);
}

fn saludar_referencia(texto: &String) {
    println!("Hola, {}", texto);
}

fn referencias_mutables() {
    println!("\n--- Referencias mutables (&mut) ---");
    let mut mensaje = String::from("Hola");
    agregar_mundo(&mut mensaje);
    println!("{}", mensaje);
}

fn agregar_mundo(texto: &mut String) {
    texto.push_str(" mundo");
}

fn una_referencia_mutable() {
    println!("\n--- Solo una referencia mutable a la vez ---");
    let mut data = String::from("Hola");
    let r1 = &mut data;
    println!("{}", r1);
    // let r2 = &mut data; // ❌ Error: solo una mutable activa
}

fn mutable_y_no_mutable() {
    println!("\n--- No se puede tener mutable y no mutable a la vez ---");
    let mut saludo = String::from("Hola");
    let r1 = &saludo;
    let r2 = &saludo;
    println!("{}, {}", r1, r2);
    // let r3 = &mut saludo; // ❌ Error: no se permite
}

fn clonacion() {
    println!("\n--- Clonación (clone) ---");
    let original = String::from("Texto");
    let copia = original.clone();
    println!("Original: {}", original);
    println!("Copia: {}", copia);
}

fn tipos_copy() {
    println!("\n--- Tipos Copy (i32, bool, char) ---");
    let x = 42;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
