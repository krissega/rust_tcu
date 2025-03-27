pub struct Mascota {
    nombre: String,
    especie: String,
    edad: u8,
}
impl Mascota {
    pub fn new(nombre: String, especie: String, edad: u8) -> Mascota {
        return Mascota {
            nombre,
            especie,
            edad,
        };
    }

    pub fn informacion(&mut self) -> String {
        return format!(
            "Nombre: {}, Especie: {}, Edad: {}",
            self.nombre, self.especie, self.edad
        );
    }
}
