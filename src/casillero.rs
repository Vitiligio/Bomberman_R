use crate::bomba_normal::BombaNormal;
use crate::bomba_super::BombaTraspaso;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::pared::Pared;
use crate::posicion::Posicion;
use crate::roca::Roca;
use crate::vacio::Vacio;

///
/// Definition of the functions that should implement every object saved in the map
/// The Trait Casillero is what composes the map. So every object stored and used by the map 
/// implements this trait
pub trait Casillero {
    fn get_simbolo(&self) -> &String;
    fn copiar_posicion(&self) -> Posicion;

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool;

    /// Calls the fuction that actually contains the logic behind hurting an object
    fn herir(&mut self, id: &str) -> Vec<Vec<Posicion>>;

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, origen: &str) -> bool;
}

impl Casillero for Enemigo {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting an Enemigo object
    fn herir(&mut self, id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar(id)
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}

impl Casillero for BombaNormal {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction to explode the bomb, as every bomb hurt should explode
    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.explotar()
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}

impl Casillero for BombaTraspaso {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction to explode the bomb, as every bomb hurt should explode
    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.explotar()
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}

impl Casillero for Vacio {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting a Vacio object
    /// For the current implementation this function could just return an empty vector
    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar()
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}

impl Casillero for Roca {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting a Roca object
    /// For the current implementation this function could just return an empty vector
    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar()
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, origen: &str) -> bool {
        let vec: Vec<char> = origen.chars().collect();
        vec[0] == 'S'
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}

impl Casillero for Pared {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting a Pared object
    /// For the current implementation this function could just return an empty vector
    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar()
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        false
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}

impl Casillero for Desvio {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Reconstructs information from the objet "hurting" the Desvio object
    /// And calls the fuctions that manages the logic of the redirecting explotion
    fn herir(&mut self, id: &str) -> Vec<Vec<Posicion>> {
        let rango: Vec<char> = id.chars().collect();
        self.desviar(
            rango[1],
            Posicion {
                x: rango[2] as usize - '0' as usize,
                y: rango[3] as usize - '0' as usize,
            },
        )
    }

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        false
    }

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}
