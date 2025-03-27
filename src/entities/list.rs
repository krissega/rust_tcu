use super::mascota::Mascota;

pub struct Lista<T> {
    elementos: Vec<T>,
}
impl<T> Lista<T> {
    pub fn new() -> Lista<T> {
        return Lista {
            elementos: Vec::new(),
        };
    }

    pub fn agregar(&mut self, elemento: T) {
        self.elementos.push(elemento);
    }

    pub fn mostrar(&self) {
        for elemento in &self.elementos {
            println!("{:?}", elemento);
        }
    }
}

fn main() {
    let mut lista = Lista<&Mascota>::new();
    let mascota = Mascota {
        nombre: String::from("Firulais"),
        edad: 5,
    };
    let mascota2 = Mascota {
        nombre: String::from("Toby"),
        edad: 3,
    };
    let mascota3 = Mascota {
        nombre: String::from("Rex"),
        edad: 7,
    };
    lista.agregar(&mascota);
    lista.agregar(&mascota2);
    lista.agregar(&mascota3);
    lista.mostrar();
}
