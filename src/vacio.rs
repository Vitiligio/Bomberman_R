use crate::posicion::Posicion;
pub struct Vacio {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Vacio {
    pub fn new(posicion_original: Posicion) -> Self {
        Self {  simbolo: "_".to_string(), 
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
