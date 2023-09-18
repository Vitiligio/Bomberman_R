mod enemigo;
mod posicion;
mod bomba_normal;
mod bomba_super;
mod casillero;
mod mapa;
mod vacio;
mod roca;
mod pared;
mod desvio;

use crate::mapa::Mapa;

use crate::posicion::Posicion;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("D:\\Bibliotecas\\Desktop\\pruebaEscritorio.txt");
    //let path = Path::new("D:\\Bibliotecas\\Desktop\\xd1.txt");
    //let path = Path::new("D:\\Bibliotecas\\Desktop\\ej3B.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let mut mapa = Mapa::new(s);
    mapa.mostrar();
    mapa.herir_objeto(Posicion { x: 4, y: 2 }, "_00".to_string());
    mapa.mostrar();

}
