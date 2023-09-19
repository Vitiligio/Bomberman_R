use crate::posicion::Posicion;
/// 
/// It is the definition of the deflect type
/// 
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
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
pub struct Desvio {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Desvio {
    ///
    /// Creates a new defect instance when providing the 'direction' and Position values
    pub fn new(direccion: char, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("D{}", direccion),
            posicion: posicion_original,
            es_vaciable: false,
        }
    }

    ///
    /// It is used to obtain a reference to the current position in the map of the defect instance
    pub fn get_posicion(&self) -> &Posicion {
        &self.posicion
    }

    fn move_down(&self, tope: usize) -> Vec<Posicion> {
        let mut vec = Vec::new();
        for i in 1..tope + 1 {
            vec.push(self.posicion.sumar((i, 0)));
        }
        vec
    }

    fn move_up(&self, tope: usize) -> Vec<Posicion> {
        let mut vec = Vec::new();
        for i in 1..tope + 1 {
            match self.posicion.check_resta((i, 0)) {
                Some(c) => vec.push(c),
                None => break,
            }
        }
        vec
    }

    fn move_right(&self, tope: usize) -> Vec<Posicion> {
        let mut vec = Vec::new();
        for i in 1..tope + 1 {
            vec.push(self.posicion.sumar((0, i)));
        }
        vec
    }

    fn move_left(&self, tope: usize) -> Vec<Posicion> {
        let mut vec = Vec::new();
        for i in 1..tope + 1 {
            match self.posicion.check_resta((0, i)) {
                Some(c) => vec.push(c),
                None => break,
            }
        }
        vec
    }

    /// This function receives the position of the bomb that reached the 'Desviar' object
    /// and the range of the explotion.
    /// With that it is calculated how long should the explotion continue and the direction is hardcoded to the 
    /// 'Desviar' object once created
    /// It returns list of lists contaning Positions but the direction will alway be just one
    /// # Example
    /// The original map
    /// ```
    /// _ _ _ _ _
    /// _ _ _ _ _
    /// _ B4 _ DU _
    /// _ _ _ _ _
    /// _ _ _ _ _
    /// ```
    /// After the explotion
    /// ```
    /// _ x _ x _
    /// _ x _ x _
    /// x _ x DU _
    /// _ x _ _ _
    /// _ x _ _ _
    /// ```
    /// See that from 'DU' there is only one row of x coming out
    pub fn desviar(&self, rango: char, posicion: Posicion) -> Vec<Vec<Posicion>> {
        let dir_desvio: Vec<char> = self.simbolo.chars().collect();
        let rango_num = rango as usize - '0' as usize;
        let x = posicion.x.abs_diff(self.posicion.x);
        let y = posicion.y.abs_diff(self.posicion.y);
        // X o Y va a valer cero, me aprovecho de eso y los sumo
        let mut cant_a_mover = 0;
        if (x + y) < rango_num {
            cant_a_mover = (x + y).abs_diff(rango_num);
        }
        let mut vec_filas = Vec::new();
        let mut vec = Vec::new();

        if dir_desvio[1] == 'D' {
            vec = self.move_down(cant_a_mover);
        } else if dir_desvio[1] == 'U' {
            vec = self.move_up(cant_a_mover);
        } else if dir_desvio[1] == 'L' {
            vec = self.move_left(cant_a_mover);
        } else if dir_desvio[1] == 'R' {
            vec = self.move_right(cant_a_mover);
        }
        if !vec.is_empty() {
            vec_filas.push(vec);
        }
        vec_filas
    }
}

#[cfg(test)]
mod tests {

    use crate::desvio::Desvio;
    use crate::posicion::Posicion;

    #[test]
    fn test_crear_desvio_hacia_arriba() {
        let desvio = Desvio::new('U', Posicion { x: 0, y: 0 });
        assert_eq!(desvio.simbolo, "DU".to_string());
    }
    #[test]
    fn test_crear_desvio_hacia_abajo() {
        let desvio = Desvio::new('D', Posicion { x: 0, y: 0 });
        assert_eq!(desvio.simbolo, "DD".to_string());
    }
    #[test]
    fn test_crear_desvio_hacia_derecha() {
        let desvio = Desvio::new('R', Posicion { x: 0, y: 0 });
        assert_eq!(desvio.simbolo, "DR".to_string());
    }
    #[test]
    fn test_crear_desvio_hacia_izquierda() {
        let desvio = Desvio::new('L', Posicion { x: 0, y: 0 });
        assert_eq!(desvio.simbolo, "DL".to_string());
    }

    #[test]
    fn test_desviar_bomba_hacia_abajo() {
        let desvio = Desvio::new('D', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar('3', Posicion { x: 5, y: 4 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_arriba() {
        let desvio = Desvio::new('U', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar('3', Posicion { x: 5, y: 4 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_derecha() {
        let desvio = Desvio::new('R', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar('3', Posicion { x: 4, y: 5 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_izquierda() {
        let desvio = Desvio::new('L', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar('3', Posicion { x: 4, y: 5 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }
}
