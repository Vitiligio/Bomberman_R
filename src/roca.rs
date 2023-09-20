use crate::posicion::Posicion;
///
/// It is the definition of the rock type
///
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
/// ```
/// The 'Posicion' field is the current position of the rock in the map
/// ```
/// posicion: Posicion,
/// ```
/// The 'es_vaciable' field says to the map if it should empty this object out of the map
/// given the correct situation
/// ```
/// pub es_vaciable: bool,
/// ```
pub struct Roca {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Roca {
    ///
    /// Creates a new rock instance when providing the 'posicion' value
    pub fn new(posicion_original: Posicion) -> Self {
        Self {
            simbolo: "R".to_string(),
            posicion: posicion_original,
            es_vaciable: false,
        }
    }

    ///
    /// Manages the logic behind hurting the rock instance
    /// For the current game logic, it only returns an empty vector
    pub fn lastimar(&mut self) -> Vec<Vec<Posicion>> {
        let vec: Vec<Vec<Posicion>> = Vec::new();
        vec
    }

    ///
    /// It is used to obtain a reference to the current position in the map of the rock instance
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }
}
