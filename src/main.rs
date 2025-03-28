mod pilas;
use pilas::PilaAritmetrica;
use pilas::PilaAritmetricaCustom;
fn main() {
    let operation_string = "3 4 + 5 *".chars().rev().collect::<String>();
    println!("{0}", operation_string);
    let operation = operation_string.split(" ");
    let mut pila_aritmetrica = PilaAritmetricaCustom::new();
    for value in operation {
        pila_aritmetrica.push(value.to_string());
    }
    println!("Size {0}", pila_aritmetrica.size());
    pila_aritmetrica.solve();
    println!("Result: {0}", pila_aritmetrica.peek().unwrap());
    println!("Size {0}", pila_aritmetrica.size())

    // let mut pila = PilaCustom::new();
    // pila.push(76);
    // pila.push(235);
    // pila.push(-90);
    // pila.push(22);
}
