use crate::posicion::Posicion;
#[derive(Clone)]
///
/// It is the definition of the explosion type
///
/// # What is inside
/// The 'id' variable shows the ID of the bomb that created the explosion
/// ```
/// id: String,
/// ```
/// The 'Posicion' field is the current position of the wall in the map
/// ```
/// posicion: Posicion,
/// ```
/// The 'rango_restante' field says to the map how many 'Rafagas' are left on this direction
/// ```
/// rango_restante: usize,
/// ```
pub struct Rafaga {
    id: String,
    rango_restante: usize,
    posicion: Posicion,
}

impl Rafaga {
    ///
    /// Creates a new explosion instance when providing the id of the bomb that created it,
    /// the range left and the position
    pub fn new(id_bomba: String, rango: usize, posicion_original: Posicion) -> Self {
        Self {
            id: id_bomba,
            rango_restante: rango,
            posicion: posicion_original,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_rango(&self) -> &usize {
        &self.rango_restante
    }
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }
}
