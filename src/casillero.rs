use crate::bomba_normal::Bomba;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::pared::Pared;
use crate::posicion::Posicion;
use crate::rafaga::Rafaga;
use crate::roca::Roca;
use crate::vacio::Vacio;

///
/// Definition of the functions that should implement every object saved in the map
/// The Trait Casillero is what composes the map. So every object stored and used by the map
/// implements this trait
pub trait Casillero {
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String;

    /// Provides a copy of the current position in the map of the object implementing this trait
    fn copiar_posicion(&self) -> Posicion;

    /// Informs to the map if the object could be removed from the map
    fn vaciable(&self) -> bool;

    /// Calls the fuction that actually contains the logic behind hurting an object
    fn herir(&mut self, rafaga: &Rafaga) -> Vec<Vec<Rafaga>>;

    /// Informs the map if the explotion of the bomb should go thru this object or not
    fn pasa_fuego_de(&mut self, origen: &str) -> bool;
}

impl Casillero for Enemigo {
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    /// Provides a copy of the current position in the map of the object
    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting an Enemigo object
    fn herir(&mut self, rafaga: &Rafaga) -> Vec<Vec<Rafaga>> {
        self.lastimar(rafaga.get_id())
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

impl Casillero for Bomba {
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    /// Provides a copy of the current position in the map of the object
    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction to explode the bomb, as every bomb hurt should explode
    fn herir(&mut self, _rafaga: &Rafaga) -> Vec<Vec<Rafaga>> {
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
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    /// Provides a copy of the current position in the map of the object
    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting a Vacio object
    /// For the current implementation this function could just return an empty vector
    fn herir(&mut self, _rafaga: &Rafaga) -> Vec<Vec<Rafaga>> {
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
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    /// Provides a copy of the current position in the map of the object
    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting a Roca object
    /// For the current implementation this function could just return an empty vector
    fn herir(&mut self, _rafaga: &Rafaga) -> Vec<Vec<Rafaga>> {
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
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    /// Provides a copy of the current position in the map of the object
    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Calls the fuction that actually contains the logic behind hurting a Pared object
    /// For the current implementation this function could just return an empty vector
    fn herir(&mut self, _rafaga: &Rafaga) -> Vec<Vec<Rafaga>> {
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
    /// Provides a reference of the current symbol that should display in the map representing
    /// the current status of the object
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    /// Provides a copy of the current position in the map of the object
    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    /// Reconstructs information from the objet "hurting" the Desvio object
    /// And calls the fuctions that manages the logic of the redirecting explotion
    fn herir(&mut self, rafaga: &Rafaga) -> Vec<Vec<Rafaga>> {
        self.desviar(rafaga.clone())
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
