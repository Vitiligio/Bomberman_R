use crate::posicion::Posicion;
use crate::rafaga::Rafaga;

///
/// It is the definition of the common bomb type
///
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
/// ```
/// The 'rango' field is the size of the explotion caused by the bomb
/// ```
/// rango: usize,
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
pub struct Bomba {
    pub simbolo: String,
    rango: usize,
    posicion: Posicion,
    pub es_vaciable: bool,
    id: String,
}

impl Bomba {
    ///
    /// Creates a new common bomb instance when providing the 'rango' and 'posicion' values
    pub fn new(tipo_bomba: char, puntos_rango: usize, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("{}{}", tipo_bomba, puntos_rango),
            rango: puntos_rango,
            posicion: posicion_original.clone(),
            es_vaciable: true,
            id: format!(
                "{}{}{}{}",
                tipo_bomba, puntos_rango, posicion_original.x, posicion_original.y
            ),
        }
    }
    ///
    /// It is used to obtain a reference to the current position in the map of the common bomb
    /// Is used to generate the ID of the bomb when damaging other objects if needed
    /// Also used to let know the map what position to empty once it explodes
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    ///
    /// This function explodes the bomb and returns a list of lists contaning Positions
    /// that continue each vector in a specific direction
    ///  # A bomb explodes in the coordinate (2, 2) with a 'rango' of 2
    /// ```
    /// let bomba_normal = BombaNormal::new(2, Posicion { x: 2, y: 2 });
    /// let vec_result = bomba_normal.explotar();
    /// ```
    /// vec_result will contain [[(1, 2), (0, 2)], [(3, 2), (4, 2)], [(2, 1), (2, 0)], [(2, 3), (0, 4)]]
    /// Seen in the map, the positions obtained will be
    /// ```
    /// _ _ x _ _
    /// _ _ x _ _
    /// x x _ x x
    /// _ _ x _ _
    /// _ _ x _ _
    /// ```
    pub fn explotar(&mut self) -> Vec<Vec<Rafaga>> {
        self.simbolo = "_".to_string();

        let mut vec_dires: Vec<Vec<Rafaga>> = Vec::new();

        let mut vec1: Vec<Rafaga> = Vec::new();
        let mut vec2: Vec<Rafaga> = Vec::new();
        let mut vec3: Vec<Rafaga> = Vec::new();
        let mut vec4: Vec<Rafaga> = Vec::new();

        for i in 1..self.rango + 1 {
            vec1.push(Rafaga::new(
                self.get_id().clone(),
                self.rango - i,
                self.posicion.sumar((i, 0)),
            ));
            vec3.push(Rafaga::new(
                self.get_id().clone(),
                self.rango - i,
                self.posicion.sumar((0, i)),
            ));
            match self.posicion.check_resta((i, 0)) {
                Some(c) => vec2.push(Rafaga::new(self.get_id().clone(), self.rango - i, c)),
                None => continue,
            }
            match self.posicion.check_resta((0, i)) {
                Some(c) => vec4.push(Rafaga::new(self.get_id().clone(), self.rango - i, c)),
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

#[cfg(test)]
mod tests {

    use crate::bomba_normal::Bomba;
    use crate::posicion::Posicion;

    #[test]
    fn test_crear_bomba_con_rango() {
        let bomba_normal = Bomba::new('B', 2, Posicion { x: 0, y: 0 });
        assert_eq!(bomba_normal.simbolo, "B2".to_string());
    }

    #[test]
    fn test_explotar_bomba_cambia_su_simbolo() {
        let mut bomba_normal = Bomba::new('B', 2, Posicion { x: 0, y: 0 });
        bomba_normal.explotar();
        assert_eq!(bomba_normal.simbolo, "_".to_string());
    }

    #[test]
    fn test_explotar_bomba_devuelve_vec_posiciones() {
        let n = 3;
        let mut bomba_normal = Bomba::new('B', n, Posicion { x: 5, y: 5 });
        let vec_posiciones_afectadas = bomba_normal.explotar();
        assert_ne!(vec_posiciones_afectadas.len() as usize, 0);
    }
}
