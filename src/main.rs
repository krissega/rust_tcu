
fn for_loop(numeros: &Vec<i32>) {
    println!("for loop:");
    for n in numeros {
        println!("Número: {}", n);
    }
}

fn while_loop(numeros: &Vec<i32>) {
    println!("\nwhile loop:");
    let mut i = 0;
    while i < numeros.len() {
        println!("Número en índice {}: {}", i, numeros[i]);
        i += 1;
    }
}

fn loop_loop(numeros: &Vec<i32>) {
    println!("\nloop + break:");
    let mut i = 0;
    loop {
        if i >= numeros.len() {
            break;
        }
        println!("Número en loop: {}", numeros[i]);
        i += 1;
    }
}

fn iter_for_each(numeros: &Vec<i32>) {
    println!("\niter().for_each:");
    numeros.iter().for_each(|n| {
        println!("Número con for_each: {}", n);
    });
}

fn iter_enumerate(numeros: &Vec<i32>) {
    println!("\niter().enumerate:");
    for (i, n) in numeros.iter().enumerate() {
        println!("Índice {}: valor {}", i, n);
    }
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5];

    for_loop(&numeros);
    while_loop(&numeros);
    loop_loop(&numeros);
    iter_for_each(&numeros);
    iter_enumerate(&numeros);
}