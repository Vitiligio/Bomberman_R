use crate::posicion::Posicion;
pub struct Desvio {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Desvio {
    pub fn new(direccion: char, posicion_original: Posicion) -> Self {
        Self {
            simbolo: format!("D{}", direccion),
            posicion: posicion_original,
            es_vaciable: false,
        }
    }

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
    
    use crate::posicion::Posicion as Posicion;
    use crate::desvio::Desvio as Desvio;

    #[test]
    fn test_crear_desvio_hacia_arriba() {
        let desvio = Desvio::new('U', Posicion { x:0, y:0 });
        assert_eq!(desvio.simbolo, "DU".to_string());
    }
    #[test]
    fn test_crear_desvio_hacia_abajo() {
        let desvio = Desvio::new('D', Posicion { x:0, y:0 });
        assert_eq!(desvio.simbolo, "DD".to_string());
    }
    #[test]
    fn test_crear_desvio_hacia_derecha() {
        let desvio = Desvio::new('R', Posicion { x:0, y:0 });
        assert_eq!(desvio.simbolo, "DR".to_string());
    }
    #[test]
    fn test_crear_desvio_hacia_izquierda() {
        let desvio = Desvio::new('L', Posicion { x:0, y:0 });
        assert_eq!(desvio.simbolo, "DL".to_string());
    }

    #[test]
    fn test_desviar_bomba_hacia_abajo() {
        let desvio = Desvio::new('D', Posicion { x:5, y:5 });
        let vec_desviado = desvio.desviar('3', Posicion { x:5, y:4 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_arriba() {
        let desvio = Desvio::new('U', Posicion { x:5, y:5 });
        let vec_desviado = desvio.desviar('3', Posicion { x:5, y:4 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_derecha() {
        let desvio = Desvio::new('R', Posicion { x:5, y:5 });
        let vec_desviado = desvio.desviar('3', Posicion { x:4, y:5 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }

    #[test]
    fn test_desviar_bomba_hacia_izquierda() {
        let desvio = Desvio::new('L', Posicion { x:5, y:5 });
        let vec_desviado = desvio.desviar('3', Posicion { x:4, y:5 });
        assert_eq!(vec_desviado.len() as usize, 1);
    }
    
}