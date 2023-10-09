use crate::posicion::Posicion;
use crate::rafaga::Rafaga;
///
/// It is the definition of the wall type
///
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
/// ```
/// The 'Posicion' field is the current position of the wall in the map
/// ```
/// posicion: Posicion,
/// ```
/// The 'es_vaciable' field says to the map if it should empty this object out of the map
/// given the correct situation
/// ```
/// pub es_vaciable: bool,
/// ```
pub struct Pared {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Pared {
    ///
    /// Creates a new wall instance when providing the 'posicion' value
    pub fn new(posicion_original: Posicion) -> Self {
        Self {
            simbolo: "W".to_string(),
            posicion: posicion_original,
            es_vaciable: false,
        }
    }

    ///
    /// Manages the logic behind hurting the wall instance
    /// For the current game logic, it only returns an empty vector
    pub fn lastimar(&mut self) -> Vec<Vec<Rafaga>> {
        let vec: Vec<Vec<Rafaga>> = Vec::new();
        vec
    }

    ///
    /// It is used to obtain a reference to the current position in the map of the wall instance
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }
}
