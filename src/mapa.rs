use crate::bomba_normal::BombaNormal;
use crate::bomba_super::BombaTraspaso;
use crate::casillero::Casillero;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::pared::Pared;
use crate::posicion::Posicion;
use crate::roca::Roca;
use crate::vacio::Vacio;
pub struct Mapa {
    pub matriz: Vec<Vec<Box<dyn Casillero>>>,
}

impl Mapa {
    pub fn new(texto: String) -> Self {
        let mut vec_filas_matriz: Vec<Vec<Box<dyn Casillero>>> = Vec::new();
        let filas: Vec<&str> = texto.split('\n').collect();
        for (indice_fila, fila) in filas.iter().enumerate() {
            let mut vec_casillero: Vec<Box<dyn Casillero>> = Vec::new();
            let vec_simbolos: Vec<&str> = fila.split(' ').collect();

            for (indice_columna, simbolo) in vec_simbolos.iter().enumerate() {
                let x: Vec<char> = simbolo.chars().collect();
                if x.is_empty() {
                    break;
                };
                vec_casillero.push(Mapa::crear_casillero(
                    x,
                    Posicion::new(indice_fila, indice_columna),
                ));
            }
            vec_filas_matriz.push(vec_casillero);
        }

        Self {
            matriz: vec_filas_matriz,
        }
    }

    fn crear_casillero(vec: Vec<char>, posicion: Posicion) -> Box<dyn Casillero> {
        if vec[0] == 'F' {
            let numero_casteado = vec[1] as usize - '0' as usize;
            Box::new(Enemigo::new(numero_casteado, posicion))
        } else if vec[0] == 'B' {
            let numero_casteado = vec[1] as usize - '0' as usize;
            Box::new(BombaNormal::new(numero_casteado, posicion))
        } else if vec[0] == 'S' {
            let numero_casteado = vec[1] as usize - '0' as usize;
            Box::new(BombaTraspaso::new(numero_casteado, posicion))
        } else if vec[0] == 'R' {
            Box::new(Roca::new(posicion))
        } else if vec[0] == 'W' {
            Box::new(Pared::new(posicion))
        } else if vec[0] == 'D' {
            Box::new(Desvio::new(vec[1], posicion))
        } else {
            Box::new(Vacio::new(posicion))
        }
    }

    pub fn mostrar(&self) {
        print!("\n\n");
        for i in &self.matriz {
            for j in i {
                print!("{} ", j.get_simbolo());
            }
            print!("\n\n");
        }
    }

    pub fn vaciar(&mut self, posicion: Posicion) {
        if self.matriz[posicion.x][posicion.y].get_simbolo() == &"_".to_string() {
            self.matriz[posicion.x][posicion.y] =
                Box::new(Vacio::new(Posicion::new(posicion.x, posicion.y)));
        }
    }

    pub fn herir_objeto(&mut self, posicion: Posicion, id: String) {
        let tamano = self.matriz.len();
        if posicion.x < tamano && posicion.y < tamano {
            let simbolo_que_hiere = &self.matriz[posicion.x][posicion.y].get_simbolo().clone();
            let repercusiones: Vec<Vec<Posicion>> = self.matriz[posicion.x][posicion.y].herir(&id);
            if self.matriz[posicion.x][posicion.y].vaciable() {
                self.vaciar(Posicion {
                    x: posicion.x,
                    y: posicion.y,
                })
            }
            let x: Vec<char> = simbolo_que_hiere.chars().collect();
            if !repercusiones.is_empty() {
                if x[0] == 'D' {
                    self.analizar_repercusiones(repercusiones, id);
                } else {
                    let id = format!("{simbolo_que_hiere}{}{}", posicion.x, posicion.y);
                    self.analizar_repercusiones(repercusiones, id);
                }
            }
        }
    }

    fn analizar_repercusiones(&mut self, vec_rep: Vec<Vec<Posicion>>, origen: String) {
        let tamano = self.matriz.len();
        for j in vec_rep {
            for i in j {
                if i.x < tamano && i.y < tamano {
                    self.herir_objeto(i.clone(), origen.clone());
                    if !self.matriz[i.x][i.y].pasa_fuego_de(&origen) {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /*
        use crate::casillero::Casillero;
        use crate::vacio::Vacio;
        use crate::enemigo::Enemigo;
        use crate::bomba_normal::BombaNormal;
        use crate::bomba_super::BombaTraspaso;
        use crate::posicion::Posicion;
        use crate::pared::Pared;
        use crate::mapa::Mapa;
        use crate::roca::Roca;

        #[test]
        fn test_bomba_normal_mata_enemigo() {
            let mut mapa = Mapa::new("B2 W F1".to_string());
            mapa.herir_objeto(Posicion::new(0, 0));
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_bomba_normal_no_atraviesa_pared() {
            let mut mapa = Mapa::new("B2 W F1".to_string());
            mapa.herir_objeto(Posicion::new(0, 0));
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
        }

        fn test_bomba_super_no_atraviesa_pared() {

        }

        fn test_bomba_normal_no_atraviesa_roca() {

        }

        fn test_bomba_super_si_atraviesa_roca() {

        }
    */
}
