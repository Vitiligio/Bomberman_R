use crate::posicion::Posicion;
pub struct BombaTraspaso {
    pub simbolo: String,
    rango: usize,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl BombaTraspaso {
    pub fn new(puntos_rango: usize, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("S{}", puntos_rango),
            rango: puntos_rango,
            posicion: posicion_original,
            es_vaciable: true,
        }
    }

    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }

    pub fn explotar(&mut self) -> Vec<Vec<Posicion>> {
        self.simbolo = "_".to_string();

        let mut vec_dires: Vec<Vec<Posicion>> = Vec::new();

        let mut vec1: Vec<Posicion> = Vec::new();
        let mut vec2: Vec<Posicion> = Vec::new();
        let mut vec3: Vec<Posicion> = Vec::new();
        let mut vec4: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1 {
            vec1.push(self.posicion.sumar((i, 0)));
            vec3.push(self.posicion.sumar((0, i)));
            match self.posicion.check_resta((i, 0)) {
                Some(c) => vec2.push(c),
                None => continue,
            }
            match self.posicion.check_resta((0, i)) {
                Some(c) => vec4.push(c),
                None => continue,
            }
        }
        if !vec1.is_empty() {
            vec_dires.push(vec1);
        }
        if !vec2.is_empty() {
            vec_dires.push(vec2);
        }
        if !vec3.is_empty() {
            vec_dires.push(vec3);
        }
        if !vec4.is_empty() {
            vec_dires.push(vec4);
        }
        vec_dires
    }
}
