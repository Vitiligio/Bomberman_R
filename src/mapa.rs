use crate::bomba_normal::Bomba;
use crate::casillero::Casillero;
use crate::desvio::Desvio;
use crate::enemigo::Enemigo;
use crate::pared::Pared;
use crate::posicion::Posicion;
use crate::rafaga::Rafaga;
use crate::roca::Roca;
use crate::vacio::Vacio;

///
/// It's the definition of the map struct that contains the macro game logic
/// It contains a matrix of objects that implement the trait 'Casillero'
/// ```
/// pub matriz: Vec<Vec<Box<dyn Casillero>>>,
/// ```
pub struct Mapa {
    pub matriz: Vec<Vec<Box<dyn Casillero>>>,
}

impl Mapa {
    /// Creates a new map instance from a String, the map has to be a square
    pub fn new(texto: String) -> Result<Self, String> {
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
                match Mapa::crear_casillero(x, Posicion::new(indice_fila, indice_columna)) {
                    Ok(t) => vec_casillero.push(t),
                    Err(e) => return Err(e),
                }
            }
            vec_filas_matriz.push(vec_casillero);
        }
        for i in vec_filas_matriz.iter() {
            if i.len() != vec_filas_matriz.len() {
                return Err("ERROR: Map should be square".to_string());
            }
        }
        Ok(Self {
            matriz: vec_filas_matriz,
        })
    }

    fn crear_casillero(vec: Vec<char>, posicion: Posicion) -> Result<Box<dyn Casillero>, String> {
        match vec[0] {
            'F' => {
                let numero_casteado = vec[1] as usize - '0' as usize;
                Ok(Box::new(Enemigo::new(numero_casteado, posicion)))
            }
            'B' => {
                let numero_casteado = vec[1] as usize - '0' as usize;
                Ok(Box::new(Bomba::new('B', numero_casteado, posicion)))
            }
            'S' => {
                let numero_casteado = vec[1] as usize - '0' as usize;
                Ok(Box::new(Bomba::new('S', numero_casteado, posicion)))
            }
            'R' => Ok(Box::new(Roca::new(posicion))),
            'W' => Ok(Box::new(Pared::new(posicion))),
            '_' => Ok(Box::new(Vacio::new(posicion))),
            'D' => {
                if vec.len() == 1 {
                    Err("ERROR: After D, it should be U, D, L or R".to_string())
                } else {
                    Ok(Box::new(Desvio::new(vec[1], posicion)))
                }
            }
            _ => Err("ERROR: Char is not recognized as valid".to_string()),
        }
    }

    ///
    /// Generates a String representation of the current map status
    /// reading the symbols of every object contained in the map and arrangeing them
    /// as each Vector being a different line
    pub fn mostrar(&self) -> String {
        let mut representacion = String::new();
        for i in &self.matriz {
            for j in i {
                representacion.push_str(j.get_simbolo());
                representacion.push(' ');
            }
            representacion.pop();
            representacion.push('\n');
        }
        representacion.pop();
        representacion
    }

    ///
    /// Receives a Position to be emptied from the map
    /// Replaces the object in that position with the object 'Vacio'
    pub fn vaciar(&mut self, posicion: Posicion) {
        if self.matriz[posicion.x][posicion.y].get_simbolo() == &"_".to_string() {
            self.matriz[posicion.x][posicion.y] =
                Box::new(Vacio::new(Posicion::new(posicion.x, posicion.y)));
        }
    }

    ///
    /// The central game logic is described here
    /// To hurt an object you have to tell the map the position to be hurt and who is hurting them
    /// The object being hurt will respond with a list of lists of positions to be hurt as a consequence
    /// This generates a recursive process that works thru the chain reactions
    /// Here is called for the 'Casilleros' to be emptied if needed
    pub fn herir_objeto(&mut self, rafaga: Rafaga) {
        let tamano = self.matriz.len();
        let posicion = rafaga.get_posicion();
        if posicion.x < tamano && posicion.y < tamano {
            let repercusiones: Vec<Vec<Rafaga>> =
                self.matriz[posicion.x][posicion.y].herir(&rafaga);
            if self.matriz[posicion.x][posicion.y].vaciable() {
                self.vaciar(Posicion {
                    x: posicion.x,
                    y: posicion.y,
                })
            }
            if !repercusiones.is_empty() {
                self.analizar_repercusiones(repercusiones);
            }
        }
    }

    fn analizar_repercusiones(&mut self, vec_rep: Vec<Vec<Rafaga>>) {
        let tamano = self.matriz.len();
        for j in vec_rep {
            for i in j {
                let posicion = i.get_posicion();
                if posicion.x < tamano && posicion.y < tamano {
                    self.herir_objeto(i.clone());
                    if !self.matriz[posicion.x][posicion.y].pasa_fuego_de(i.get_id()) {
                        break;
                    }
                }
            }
        }
    }

    pub fn there_is_a_bomb_at(&self, posicion: Posicion) -> bool {
        let tamano = self.matriz.len();
        let mut resultado = false;

        if posicion.x < tamano && posicion.y < tamano {
            let simbolo_a_herir = self.matriz[posicion.x][posicion.y].get_simbolo().clone();
            let x: Vec<char> = simbolo_a_herir.chars().collect();
            if x[0] == 'S' || x[0] == 'B' {
                resultado = true;
            }
        }
        resultado
    }
}

#[cfg(test)]
mod tests {
    use crate::mapa::Mapa;
    use crate::posicion::Posicion;
    use crate::rafaga::Rafaga;

    #[test]
    fn test_bomba_normal_hiere_enemigo_y_no_lo_mata() {
        let maps = Mapa::new("B2 _ F2\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_bomba_normal_hiere_enemigo_lo_mata() {
        let maps = Mapa::new("B2 _ F1\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
    }

    #[test]
    fn test_bomba_normal_no_pasa_pared() {
        let maps = Mapa::new("B2 W F1\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_bomba_super_no_pasa_pared() {
        let maps = Mapa::new("S2 W F1\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_bomba_normal_no_pasa_roca() {
        let maps = Mapa::new("B2 R F1\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_bomba_super_pasa_pared() {
        let maps = Mapa::new("S2 R F1\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
    }

    #[test]
    fn test_bomba_explota_otra_bomba_y_esta_hiere_al_enemigo() {
        let maps = Mapa::new("B2 _ B2\n_ _ F2\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[1][2].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_bomba_desencadena_dos_explosiones_mas_que_dañan_al_enemigo() {
        let maps = Mapa::new("B2 _ B3\n_ _ F2\n_ _ B2".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[1][2].get_simbolo(), &"_".to_string());
    }

    #[test]
    fn test_bomba_se_desvia_y_lastima_al_enemigo() {
        let maps = Mapa::new("B8 _ DD\n_ _ _\n_ _ F2".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[2][2].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_misma_bomba_alcanza_dos_veces_al_enemigo_y_lo_hiere_una_vez() {
        let maps = Mapa::new("B8 F2 DL\n_ _ _\n_ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][1].get_simbolo(), &"F1".to_string());
    }

    #[test]
    fn test_ejemplo_enunciado_uno() {
        let maps = Mapa::new("B2 R R _ F1 _ _\n_ W R W _ W _\nB5 _ _ _ B2 _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
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
        let maps = Mapa::new("_ _ B2 _ B1 _ _\n_ W _ W _ W _\n_ _ B2 R F1 _ _\n_ W _ W R W _\n_ _ B4 _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ B1".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 4, y: 2 }));
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
        let maps = Mapa::new("_ _ _ _ _ _ _\n_ W _ W _ W _\nS4 R R R F2 _ _\n_ W _ W _ W _\nB2 _ B5 _ DU _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 4, y: 2 }));
        assert_eq!(mapa.matriz[4][0].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[4][2].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][4].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][1].get_simbolo(), &"R".to_string());
        assert_eq!(mapa.matriz[2][2].get_simbolo(), &"R".to_string());
        assert_eq!(mapa.matriz[2][3].get_simbolo(), &"R".to_string());
    }

    #[test]
    fn test_correccion_rafaga_debe_daniar_una_vez() {
        let maps = Mapa::new("_ _ B7 _ _ _ _\n_ W _ W _ W _\n_ _ DR F3 DL _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 2 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][3].get_simbolo(), &"F2".to_string());
    }

    #[test]
    fn test_correccion_bomba_en_cadena_1_c() {
        let maps = Mapa::new("B2 _ B2 _ _ _ _\n_ W _ W _ W _\n_ _ B4 _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ B3 _ _ _ B5".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][0].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][2].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][6].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[6][6].get_simbolo(), &"B5".to_string());
    }

    #[test]
    fn test_correccion_impactado_por_tres_bombas() {
        let maps = Mapa::new("B2 _ B2 _ _ _ _\n_ W _ W _ W _\nB1 B2 F3 B2 _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 0 }));
        assert_eq!(mapa.matriz[0][0].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][0].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][1].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][3].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][2].get_simbolo(), &"_".to_string());
    }

    #[test]
    fn test_correccion_dos_desvios_enfrentados() {
        let maps = Mapa::new("_ _ B7 _ _ _ _\n_ W _ W _ W _\n_ _ DR DL _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _\n_ W _ W _ W _\n_ _ _ _ _ _ _".to_string());
        let mut mapa = maps.unwrap();
        mapa.herir_objeto(Rafaga::new("_".to_string(), 2, Posicion { x: 0, y: 2 }));
        assert_eq!(mapa.matriz[0][2].get_simbolo(), &"_".to_string());
        assert_eq!(mapa.matriz[2][2].get_simbolo(), &"DR".to_string());
        assert_eq!(mapa.matriz[2][3].get_simbolo(), &"DL".to_string());
    }
}
