
fn if_let() {
    let valor = Some(10);
    // Usamos `if let` para extraer el valor si es Some
    if let Some(numero) = valor {
        println!("El número es: {}", numero);
    } else {
        println!("No hay número");
    }
}

fn match_conditional() {
    let nombre: Option<&str> = Some("Ana");
    match nombre {
        Some(n) => println!("Hola, {}!", n),
        None => println!("Hola, invitado!"),
    }
}

fn unwrap_example() {
    let edad: Option<u32> = None;
    // Si hay un valor, se usa; si no, se usa 18 por defecto
    let edad_final = edad.unwrap_or(18);
    println!("La edad es: {}", edad_final);
}

fn if_boolean() {
    let clave: Option<&str> = Some("secreta123");
    if clave.is_some() && clave.unwrap().len() > 5 {
        println!("Clave válida");
    } else {
        println!("Clave muy corta o no existe");
    }
}

fn some() {
    let valor: Option<i32> = Some(42);

    if let Some(x) = valor {
        println!("Usando `if let`: el valor es {}", x);
    }
}

fn and_then(){
    let numero: Option<i32> = Some(10);

    let resultado: Option<i32> = numero.and_then(|x| {
        if x > 5 {
            Some(x * 2) // sigue la cadena
        } else {
            None // se corta la cadena
        }
    });

    println!("Usando `and_then`: {:?}", resultado); // Some(20)
}

fn is_some() {
    let token: Option<&str> = Some("abc123");

    if token.is_some() {
        println!("Usando `is_some()`: Hay token");
    } else {
        println!("Usando `is_some()`: No hay token");
    }
}

fn main() {
    if_let();
    match_conditional();
    unwrap_example();
    if_boolean();
    some();
    and_then();
}
