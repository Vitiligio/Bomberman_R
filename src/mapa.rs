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

    pub fn mostrar(&self) -> String {
        let mut representacion = String::new();
        for i in &self.matriz {
            for j in i {
                representacion.push_str(j.get_simbolo());
                representacion.push(' ');
            }
            representacion.push('\n');
        }
        representacion
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
        use crate::posicion::Posicion;
        use crate::mapa::Mapa;

        #[test]
        fn test_bomba_normal_hiere_enemigo_y_no_lo_mata() {
            let mut mapa = Mapa::new("B2 _ F2\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_bomba_normal_hiere_enemigo_lo_mata() {
            let mut mapa = Mapa::new("B2 _ F1\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
        }

        #[test]
        fn test_bomba_normal_no_pasa_pared() {
            let mut mapa = Mapa::new("B2 W F1\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_bomba_super_no_pasa_pared() {
            let mut mapa = Mapa::new("S2 W F1\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_bomba_normal_no_pasa_roca() {
            let mut mapa = Mapa::new("B2 R F1\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_bomba_super_pasa_pared() {
            let mut mapa = Mapa::new("S2 R F1\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
        }

        #[test]
        fn test_bomba_explota_otra_bomba_y_esta_hiere_al_enemigo() {
            let mut mapa = Mapa::new("B2 0 B2\n_ _ F2\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[1][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_bomba_desencadena_dos_explosiones_mas_que_dañan_al_enemigo() {
            let mut mapa = Mapa::new("B2 0 B3\n_ _ F2\n_ _ B2".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[1][2].get_simbolo(), &"_".to_string());
        }

        #[test]
        fn test_bomba_se_desvia_y_lastima_al_enemigo() {
            let mut mapa = Mapa::new("B8 _ DD\n_ _ _\n_ _ F2".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[2][2].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_misma_bomba_alcanza_dos_veces_al_enemigo_y_lo_hiere_una_vez() {
            let mut mapa = Mapa::new("B8 F2 DL\n_ _ _\n_ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][1].get_simbolo(), &"F1".to_string());
        }

        #[test]
        fn test_ejemplo_enunciado_uno() {
            let mut mapa = Mapa::new("B2 R R _ F1 _ _\n_ W R W _ W _\nB5 _ _ _ B2 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
            mapa.herir_objeto(Posicion::new(0, 0), "_34".to_string());
            assert_eq!(mapa.matriz[0][0].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[2][0].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[2][4].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[0][4].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[0][1].get_simbolo(), &"R".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"R".to_string());
            assert_eq!(mapa.matriz[1][2].get_simbolo(), &"R".to_string());
        }

        #[test]
        fn test_ejemplo_enunciado_dos() {
            let mut mapa = Mapa::new("_ _ B2 _ B1 _ _\n_ W _ W _ W _\n_ _ B2 R F1 _ _\n_ W _ W R W _\n_ _ B4 _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1".to_string());
            mapa.herir_objeto(Posicion::new(4, 2), "_34".to_string());
            assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[0][4].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[2][2].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[4][2].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[2][4].get_simbolo(), &"F1".to_string());
            assert_eq!(mapa.matriz[6][6].get_simbolo(), &"B1".to_string());
            assert_eq!(mapa.matriz[2][3].get_simbolo(), &"R".to_string());
            assert_eq!(mapa.matriz[3][4].get_simbolo(), &"R".to_string());
        }

        #[test]
        fn test_ejemplo_enunciado_tres() {
            let mut mapa = Mapa::new("_ _ _ _ _ _ _\n_ W _ W _ W _\nS4 R R R F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
            mapa.herir_objeto(Posicion::new(4, 2), "_34".to_string());
            assert_eq!(mapa.matriz[2][0].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[4][0].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[4][2].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[2][4].get_simbolo(), &"_".to_string());
            assert_eq!(mapa.matriz[2][1].get_simbolo(), &"R".to_string());
            assert_eq!(mapa.matriz[2][2].get_simbolo(), &"R".to_string());
            assert_eq!(mapa.matriz[2][3].get_simbolo(), &"R".to_string());
        }

}
