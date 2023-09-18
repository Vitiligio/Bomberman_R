use crate::enemigo::Enemigo;
use crate::bomba_normal::BombaNormal;
use crate::posicion::Posicion;
use crate::vacio::Vacio;
use crate::bomba_super::BombaTraspaso;
use crate::pared::Pared;
use crate::roca::Roca;
use crate::desvio::Desvio;
pub trait Casillero {
    fn get_simbolo(&self) -> &String;
    fn copiar_posicion(&self) -> Posicion;
    fn vaciable(&self) -> bool;
    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>;
    fn pasa_fuego_de(&mut self, origen: &String) -> bool;
   // fn copiar(&self) -> Box<dyn Casillero>;
}

impl Casillero for Enemigo {
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        self.lastimar(id)
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        true
    }

    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}

impl Casillero for BombaNormal {
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        self.explotar()
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        true
    }

    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}

impl Casillero for BombaTraspaso { 
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        self.explotar()
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        true
    }
    
    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}

impl Casillero for Vacio {
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        self.lastimar()
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        true
    }
    
    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}

impl Casillero for Roca {
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        self.lastimar()
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        let vec: Vec<char> = origen.chars().collect();
        if vec[0] == 'S' { true }
        else { false }
    }
    
    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}

impl Casillero for Pared {
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        self.lastimar()
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        false
    }
    
    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}

impl Casillero for Desvio {
    fn get_simbolo(&self) -> &String{
        &self.simbolo
    }

    fn copiar_posicion(&self) -> Posicion{
        self.get_posicion().clone()
    }

    fn herir(&mut self, id: &String) -> Vec<Vec<Posicion>>{
        print!("{:?}", id);
        let rango: Vec<char> = id.chars().collect();
        self.desviar(rango[1], Posicion { x: rango[2] as usize - '0' as usize, y: rango[3] as usize - '0' as usize })
    }

    fn pasa_fuego_de(&mut self, origen: &String) -> bool {
        false
    }
    
    fn vaciable(&self) -> bool{
        self.es_vaciable
    }
}