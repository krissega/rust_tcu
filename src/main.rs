use std::thread;
use std::time::Duration;

fn main() {
    let hilo1 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(4));
        println!("Hilo 1 finalizado");
    });

    let hilo2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        println!("Hilo 2 finalizado");
    });

    let hilo3 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(6));
        println!("Hilo 3 finalizado");
    });

    // Esperamos que todos los hilos terminen
    hilo1.join().unwrap();
    hilo2.join().unwrap();
    hilo3.join().unwrap();

    println!("Todos los hilos han finalizado");
}