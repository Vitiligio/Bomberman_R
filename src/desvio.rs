use crate::posicion::Posicion;
use crate::rafaga::Rafaga;

///
/// It is the definition of the deflect type
///
/// # What is inside
/// The 'simbolo' variable is how it shows on the map
/// ```
/// pub simbolo: String,
/// ```
/// The 'Posicion' field is the current position of the deflect in the map
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

    fn move_down(&self, rafaga: Rafaga) -> Vec<Rafaga> {
        let mut vec: Vec<Rafaga> = Vec::new();
        for i in 1..rafaga.get_rango() + 1 {
            vec.push(Rafaga::new(
                rafaga.get_id().clone(),
                rafaga.get_rango() - i,
                self.posicion.sumar((i, 0)),
            ));
        }
        vec
    }

    fn move_up(&self, rafaga: Rafaga) -> Vec<Rafaga> {
        let mut vec: Vec<Rafaga> = Vec::new();
        for i in 1..rafaga.get_rango() + 1 {
            match self.posicion.check_resta((i, 0)) {
                Some(c) => vec.push(Rafaga::new(
                    rafaga.get_id().clone(),
                    rafaga.get_rango() - i,
                    c,
                )),
                None => continue,
            }
        }
        vec
    }

    fn move_right(&self, rafaga: Rafaga) -> Vec<Rafaga> {
        let mut vec: Vec<Rafaga> = Vec::new();
        for i in 1..rafaga.get_rango() + 1 {
            vec.push(Rafaga::new(
                rafaga.get_id().clone(),
                rafaga.get_rango() - i,
                self.posicion.sumar((0, i)),
            ));
        }
        vec
    }

    fn move_left(&self, rafaga: Rafaga) -> Vec<Rafaga> {
        let mut vec: Vec<Rafaga> = Vec::new();
        for i in 1..rafaga.get_rango() + 1 {
            match self.posicion.check_resta((0, i)) {
                Some(c) => vec.push(Rafaga::new(
                    rafaga.get_id().clone(),
                    rafaga.get_rango() - i,
                    c,
                )),
                None => continue,
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
    pub fn desviar(&self, rafaga: Rafaga) -> Vec<Vec<Rafaga>> {
        let dir_desvio: Vec<char> = self.simbolo.chars().collect();

        let mut vec_filas: Vec<Vec<Rafaga>> = Vec::new();
        let mut vec: Vec<Rafaga> = Vec::new();

        if dir_desvio[1] == 'D' {
            vec = self.move_down(rafaga);
        } else if dir_desvio[1] == 'U' {
            vec = self.move_up(rafaga);
        } else if dir_desvio[1] == 'L' {
            vec = self.move_left(rafaga);
        } else if dir_desvio[1] == 'R' {
            vec = self.move_right(rafaga);
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
    use crate::rafaga::Rafaga;

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
        let vec_desviado = desvio.desviar(Rafaga::new("_".to_string(), 5, Posicion { x: 5, y: 4 }));
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_arriba() {
        let desvio = Desvio::new('U', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar(Rafaga::new("_".to_string(), 5, Posicion { x: 5, y: 4 }));
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_derecha() {
        let desvio = Desvio::new('R', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar(Rafaga::new("_".to_string(), 5, Posicion { x: 4, y: 5 }));
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_izquierda() {
        let desvio = Desvio::new('L', Posicion { x: 5, y: 5 });
        let vec_desviado = desvio.desviar(Rafaga::new("_".to_string(), 5, Posicion { x: 4, y: 5 }));
        assert_eq!(vec_desviado.len() as usize, 1);
    }
}
