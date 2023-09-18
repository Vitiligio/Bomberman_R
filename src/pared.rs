use crate::posicion::Posicion;
pub struct Pared {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Pared {
    pub fn new(posicion_original: Posicion) -> Self {
        Self {  simbolo: "W".to_string(), 
                posicion: posicion_original,
                es_vaciable: false }
    }

    pub fn lastimar(&mut self) -> Vec<Vec<Posicion>>{
        let vec: Vec<Vec<Posicion>> = Vec::new();
        vec
    }
    
    pub fn get_posicion(&self) -> &Posicion{
        &self.posicion
    }
}