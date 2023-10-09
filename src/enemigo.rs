use crate::posicion::Posicion;
use crate::rafaga::Rafaga;
///
/// It is the definition of the enemy type
///
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
/// ```
/// The 'vida' field is how many bombs can the enemy take before dying
/// ```
/// vida: usize,
/// ```
/// The 'Posicion' field is the current position of the bomb in the map
/// ```
/// posicion: Posicion,
/// ```
/// The 'es_vaciable' field says to the map if it should empty this object out of the map
/// given the correct situation
/// ```
/// pub es_vaciable: bool,
/// ```
/// The 'id_bombas' field contains the history of bombs that damaged this object
/// so one bomb cannot damage more than once the same enemy
/// ```
/// id_bombas: Vec<String>,
/// ```
pub struct Enemigo {
    pub simbolo: String,
    vida: usize,
    posicion: Posicion,
    pub es_vaciable: bool,
    id_bombas: Vec<String>,
}

impl Enemigo {
    ///
    /// Creates a new enemy instance when providing the 'vida' and 'posicion' values
    pub fn new(puntos_vida: usize, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("F{}", puntos_vida),
            vida: puntos_vida,
            posicion: posicion_original,
            es_vaciable: true,
            id_bombas: Vec::new(),
        }
    }

    ///
    /// It is used to obtain a reference to the current position in the map of the enemy instance
    /// Also used to let know the map what position to empty once it dies
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }

    /// Manages the getting hurt logic of an enemy
    /// If the life value reaches zero it turns the symbol of the enemy to '_'
    /// So the map knows to empty out the 'Casillero' in that position
    /// It also updates the vec of bombs that hurt him, checking before updating the life points if the
    /// ID was already saved.
    /// It returns an empty vector always as the death of the enemy does not affect other 'Casilleros'
    pub fn lastimar(&mut self, id: &str) -> Vec<Vec<Rafaga>> {
        if !self.fue_herido_por(id) {
            self.vida -= 1;
            self.simbolo = format!("F{}", self.vida);
            if self.vida == 0 {
                self.simbolo = "_".to_string();
            }
        }
        let vec: Vec<Vec<Rafaga>> = Vec::new();
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

    use crate::casillero::Casillero;
    use crate::enemigo::Enemigo;
    use crate::posicion::Posicion;

    #[test]
    fn test_obtener_posicion_enemigo() {
        let enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        let posicion_real: &Posicion = enem.get_posicion();
        assert_eq!(posicion_real.x, 1);
        assert_eq!(posicion_real.y, 2);
    }

    #[test]
    fn obtener_simbolo_enemigo() {
        let enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        assert_eq!(enem.simbolo, "F2");
    }

    #[test]
    fn test_herir_enemigo() {
        let mut enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        enem.lastimar("_-_");
        assert_eq!(enem.simbolo, "F1");
    }

    #[test]
    fn test_herir_enemigo_y_matarlo() {
        let mut enem: Enemigo = Enemigo::new(2, Posicion { x: 1, y: 2 });
        enem.lastimar("_-_");
        enem.lastimar("--_");
        assert_eq!(enem.get_simbolo(), &"_".to_string());
    }
}
