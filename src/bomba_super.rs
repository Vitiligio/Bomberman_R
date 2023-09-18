use crate::posicion::Posicion;
pub struct BombaTraspaso {
    pub simbolo: String,
    rango: usize,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl BombaTraspaso {
    pub fn new(puntos_rango: usize, posicion_original: Posicion) -> Self {
        Self {  simbolo: format!("S{}", 
                puntos_rango), 
                rango: puntos_rango, 
                posicion: posicion_original,
                es_vaciable: true }
    }

    pub fn get_posicion(&self) -> &Posicion{
        &self.posicion
    }

    pub fn explotar(&mut self) -> Vec<Vec<Posicion>>{
        self.simbolo = "_".to_string();
        let mut vec_dires: Vec<Vec<Posicion>> = Vec::new();
        let mut vec: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1  {
            vec.push(self.posicion.sumar((i, 0)));
        }
        if vec.len() > 0 { 
            vec_dires.push(vec); 
        }
        let mut vec: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1  {
            match self.posicion.check_resta((i, 0)){
                Some(c) => vec.push(c),
                None => break,
            }
        }  
        if vec.len() > 0 { 
            vec_dires.push(vec); 
        }
        let mut vec: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1  {
            vec.push(self.posicion.sumar((0, i)));
        }
        if vec.len() > 0 { 
            vec_dires.push(vec); 
        }
        let mut vec: Vec<Posicion> = Vec::new();

        for i in 1..self.rango + 1  {
            match self.posicion.check_resta((0, i)){
                Some(c) => vec.push(c),
                None => break,
            }
        }
        if vec.len() > 0 { 
            vec_dires.push(vec);
        }
        
        vec_dires
    }
}
