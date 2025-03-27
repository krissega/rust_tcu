mod entities;

use entities::mascota::Mascota;
fn main() {
    let mut mascota: Mascota = Mascota::new("Firulais".to_string(), "Perro".to_string(), 5);
    println!("{}", &mascota.informacion());
}
