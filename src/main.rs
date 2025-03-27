use std::ptr;

struct Nodo {
    valor: i32,
    siguiente: Option<Box<Nodo>>,
    anterior: *mut Nodo, // puntero crudo, para enlazar hacia atrás
}

pub struct Cola {
    cabeza: Option<Box<Nodo>>,
    cola: *mut Nodo, // puntero a la cola para inserción rápida
}

impl Cola {
    pub fn nueva() -> Self {
        Cola {
            cabeza: None,
            cola: ptr::null_mut(),
        }
    }
    pub fn esta_vacia(&self) -> bool {
        self.cabeza.is_none()
    }

    pub fn agregar(&mut self, valor: i32) {
        let mut nuevo = Box::new(Nodo {
            valor,
            siguiente: None,
            anterior: ptr::null_mut(),
        });

        let nuevo_ptr: *mut Nodo = &mut *nuevo;

        if self.cabeza.is_none() {
            // Primer nodo
            self.cabeza = Some(nuevo);
            self.cola = nuevo_ptr;
        } else {
            unsafe {
                (*self.cola).siguiente = Some(nuevo);
                let anterior = self.cola;
                self.cola = nuevo_ptr;
                (*self.cola).anterior = anterior;
            }
        }
    }

    pub fn eliminar(&mut self) -> Option<i32> {
        self.cabeza.take().map(|mut nodo| {
            self.cabeza = nodo.siguiente.take();
            if let Some(ref mut nueva_cabeza) = self.cabeza {
                nueva_cabeza.anterior = ptr::null_mut();
            } else {
                self.cola = ptr::null_mut(); // la cola quedó vacía
            }
            nodo.valor
        })
    }
}

fn main() {
    let mut cola = Cola::nueva();

    cola.agregar(10);
    cola.agregar(20);
    cola.agregar(30);

    while !cola.esta_vacia() {
        println!("Eliminado: {}", cola.eliminar().unwrap());
    }
}
