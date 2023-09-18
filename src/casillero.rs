use crate::bomba_normal::BombaNormal;
use crate::bomba_super::BombaTraspaso;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::pared::Pared;
use crate::posicion::Posicion;
use crate::roca::Roca;
use crate::vacio::Vacio;
pub trait Casillero {
    fn get_simbolo(&self) -> &String;
    fn copiar_posicion(&self) -> Posicion;
    fn vaciable(&self) -> bool;
    fn herir(&mut self, id: &str) -> Vec<Vec<Posicion>>;
    fn pasa_fuego_de(&mut self, origen: &str) -> bool;
    // fn copiar(&self) -> Box<dyn Casillero>;
}

impl Casillero for Enemigo {
    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion {
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar(id)
    }

    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

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

    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.explotar()
    }

    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

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

    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.explotar()
    }

    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

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

    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar()
    }

    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        true
    }

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

    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar()
    }

    fn pasa_fuego_de(&mut self, origen: &str) -> bool {
        let vec: Vec<char> = origen.chars().collect();
        vec[0] == 'S'
    }

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

    fn herir(&mut self, _id: &str) -> Vec<Vec<Posicion>> {
        self.lastimar()
    }

    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        false
    }

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

    fn pasa_fuego_de(&mut self, _origen: &str) -> bool {
        false
    }

    fn vaciable(&self) -> bool {
        self.es_vaciable
    }
}
