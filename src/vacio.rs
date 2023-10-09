use crate::posicion::Posicion;
use crate::rafaga::Rafaga;
///
/// It is the definition of the empty type
///
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
/// ```
/// The 'Posicion' field is the current position of the empty space in the map
/// ```
/// posicion: Posicion,
/// ```
/// The 'es_vaciable' field says to the map if it should empty this object out of the map
/// given the correct situation
/// ```
/// pub es_vaciable: bool,
/// ```
pub struct Vacio {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Vacio {
    ///
    /// Creates a new empty instance when providing the 'posicion' value
    pub fn new(posicion_original: Posicion) -> Self {
        Self {
            simbolo: "_".to_string(),
            posicion: posicion_original,
            es_vaciable: false,
        }
    }

    ///
    /// Manages the logic behind hurting the empty instance
    /// For the current game logic, it only returns an empty vector
    pub fn lastimar(&mut self) -> Vec<Vec<Rafaga>> {
        let vec: Vec<Vec<Rafaga>> = Vec::new();
        vec
    }

    ///
    /// It is used to obtain a reference to the current position in the map of the empty instance
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }
}
