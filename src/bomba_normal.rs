use crate::posicion::Posicion;
pub struct BombaNormal {
    pub simbolo: String,
    rango: usize,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl BombaNormal {
    pub fn new(puntos_rango: usize, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("B{}", puntos_rango),
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

        for i in 1..self.rango + 1 {
            vec1.push(self.posicion.sumar((i, 0)));
        }
        if !vec1.is_empty() {
            vec_dires.push(vec1);
        }
        let mut vec2: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1 {
            match self.posicion.check_resta((i, 0)) {
                Some(c) => vec2.push(c),
                None => break,
            }
        }
        if !vec2.is_empty() {
            vec_dires.push(vec2);
        }
        let mut vec3: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1 {
            vec3.push(self.posicion.sumar((0, i)));
        }
        if !vec3.is_empty() {
            vec_dires.push(vec3);
        }
        let mut vec4: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1 {
            match self.posicion.check_resta((0, i)) {
                Some(c) => vec4.push(c),
                None => break,
            }
        }
        if !vec4.is_empty() {
            vec_dires.push(vec4);
        }

        vec_dires
    }
}

#[cfg(test)]
mod tests {
    /*
    use crate::posicion::Posicion as Posicion;
    use crate::bomba_normal::BombaNormal as BombaNormal;

    #[test]
    fn test_crear_bomba_con_rango() {
        let bomba_normal = BombaNormal::new(2, Posicion { x:0, y:0 });
        assert_eq!(bomba_normal.get_simbolo(), &"B2".to_string());
    }

    #[test]
    fn test_explotar_bomba_cambia_su_simbolo() {
        let mut bomba_normal = BombaNormal::new(2, Posicion { x:0, y:0 });
        bomba_normal.explotar();
        assert_eq!(bomba_normal.get_simbolo(), &"_".to_string());
    }

    #[test]
    fn test_explotar_bomba_devuelve_vec_posiciones() {
        let n = 3;
        let mut bomba_normal = BombaNormal::new(n, Posicion { x:5, y:5 });
        let vec_posiciones_afectadas = bomba_normal.explotar();
        assert_eq!(vec_posiciones_afectadas.len() as usize, n*4);
    }
    */
}
