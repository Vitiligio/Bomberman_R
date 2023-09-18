use crate::posicion::Posicion as Posicion;
pub struct Desvio {
    pub simbolo: String,
    posicion: Posicion,
    pub es_vaciable: bool,
}

impl Desvio {
    pub fn new(direccion: char, posicion_original: Posicion) -> Self {
        Self { simbolo: format!("D{}", direccion), 
        posicion: posicion_original,
        es_vaciable: false }
    }

    fn get_simbolo(&self) -> &String {
        &self.simbolo
    }

    pub fn get_posicion(&self) -> &Posicion{
        &self.posicion
    }

    pub fn desviar(&self, rango: char, posicion: Posicion) -> Vec<Vec<Posicion>> {
        let dir_desvio: Vec<char> = self.simbolo.chars().collect();
        let rango_num = rango as usize - '0' as usize;
        let x = posicion.x.abs_diff(self.posicion.x);
        let y = posicion.y.abs_diff(self.posicion.y);
        // X o Y va a valer cero, me aprovecho de eso y los sumo
        let cant_a_mover = (x+y).abs_diff(rango_num);
        println!("{}", rango_num);
        let mut vec_filas = Vec::new();
        let mut vec = Vec::new();

        if dir_desvio[1] == 'D' {
            for i in 1..cant_a_mover+1 {
                vec.push(self.posicion.sumar((i, 0)));
            }
            if !vec.is_empty() { vec_filas.push(vec); }
        }

        else if dir_desvio[1] == 'U' {
            for i in 1..cant_a_mover+1  {
                match self.posicion.check_resta((i, 0)){
                    Some(c) => vec.push(c),
                    None => break,
                }
            }  
            if !vec.is_empty() { vec_filas.push(vec); }
        }

        else if dir_desvio[1] == 'L' {
            for i in 1..cant_a_mover+1  {
                match self.posicion.check_resta((0, i)){
                    Some(c) => vec.push(c),
                    None => break,
                }
            }  
            if !vec.is_empty() { vec_filas.push(vec); }
        }

        else if dir_desvio[1] == 'R' {
            for i in 1..cant_a_mover+1 {
                vec.push(self.posicion.sumar((0, i)));
            }
            if !vec.is_empty() { vec_filas.push(vec); }
        }
        vec_filas        
    }
}