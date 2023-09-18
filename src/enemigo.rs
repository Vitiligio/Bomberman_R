use crate::posicion::Posicion;
pub struct Enemigo {
    pub simbolo: String,
    vida: usize,
    posicion: Posicion,
    pub es_vaciable: bool,
    id_bombas: Vec<String>,
}

impl Enemigo {
    pub fn new(puntos_vida: usize, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("F{}", puntos_vida),
            vida: puntos_vida,
            posicion: posicion_original,
            es_vaciable: true,
            id_bombas: Vec::new(),
        }
    }

    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }

    pub fn lastimar(&mut self, id: &str) -> Vec<Vec<Posicion>> {
        if !self.fue_herido_por(id) {
            self.vida -= 1;
            self.simbolo = format!("F{}", self.vida);
            if self.vida == 0 {
                self.simbolo = "_".to_string();
            }
        }
        let vec: Vec<Vec<Posicion>> = Vec::new();
        vec
    }

    fn fue_herido_por(&mut self, id: &str) -> bool {
        let mut resultado = false;
        for i in self.id_bombas.iter() {
            if i == id {
                resultado = true;
                break;
            }
        }
        if !resultado {
            self.id_bombas.push(id.to_string());
        }
        resultado
    }
}

#[cfg(test)]
mod tests {
    /*
    use crate::enemigo::Enemigo as Enemigo;
    use crate::posicion::Posicion as Posicion;


    #[test]
    fn test_obtener_posicion_enemigo() {
        let mut enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        let posicion_real: &Posicion = enem.get_posicion();
        assert_eq!(posicion_real.x, 1);
        assert_eq!(posicion_real.y, 2);
    }

    #[test]
    fn test_herir_enemigo() {
        let mut enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        enem.lastimar();
        assert_eq!(enem.get_vida(), &1);
    }

    #[test]
    fn test_herir_enemigo_y_actualizar_simbolo() {
        let mut enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        enem.lastimar();
        assert_eq!(enem.get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn obtener_simbolo_enemigo() {
        let enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        assert_eq!(enem.get_simbolo(), &"F2".to_string());
    }

    #[test]
    fn test_herir_enemigo_y_matarlo() {
        let mut enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        enem.lastimar();
        enem.lastimar();
        assert_eq!(enem.get_simbolo(), &"_".to_string());
    }

    */
}
