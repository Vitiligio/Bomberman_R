mod bomba_normal;
mod bomba_super;
mod casillero;
mod desvio;
mod enemigo;
mod mapa;
mod pared;
mod posicion;
mod roca;
mod vacio;

use crate::mapa::Mapa;

use crate::posicion::Posicion;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn escribir_archivo(path: &Path, contenido: String) {
    // Create the output file
    let display = path.display();

    if let Ok(file_output) = File::create(path) {
        let mut file = file_output;
        match file.write_all(contenido.as_bytes()) {
            Err(why) => println!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("succesfull write to {}", display),
        }
    }
}

fn main() {
    // Catch the arguments sent via the command line
    let args: Vec<String> = env::args().collect();

    // Create a path to the desired input file
    let path = Path::new(&args[1]);

    // Create a path to the desired output file
    let dir_output = Path::new(&args[2]);

    let dir_x: usize;
    let dir_y: usize;

    let map_reading: String;

    if let Ok(ter_arg) = args[3].parse::<usize>() {
        dir_x = ter_arg;
    } else {
        let contenido = "ERROR: Could not parse first argument".to_string();
        escribir_archivo(dir_output, contenido);
        return;
    }

    if let Ok(cuart_arg) = args[4].parse::<usize>() {
        dir_y = cuart_arg;
    } else {
        let contenido = "ERROR: Could not parse second argument".to_string();
        escribir_archivo(dir_output, contenido);
        return;
    }

    // Open the path in read-only mode, returns `io::Result<File>`
    if let Ok(file) = File::open(path) {
        let mut s = String::new();
        let mut read_file = file;
        if let Ok(_x) = read_file.read_to_string(&mut s) {
            map_reading = s;
        } else {
            let contenido = "ERROR: Could not read file".to_string();
            escribir_archivo(dir_output, contenido);
            return;
        }
    } else {
        let contenido = "ERROR: Could not open file".to_string();
        escribir_archivo(dir_output, contenido);
        return;
    }

    // Create the map from the contents of the file, active the bomb in the coordinates provided
    // and generated the output in a string
    if let Ok(maps) = Mapa::new(map_reading.clone()) {
        let mut mapa = maps;
        if mapa.there_is_a_bomb_at(Posicion { x: dir_y, y: dir_x }) {
            mapa.herir_objeto(Posicion { x: dir_y, y: dir_x }, "_00".to_string());
            let output = mapa.mostrar();
            escribir_archivo(dir_output, output);
        } else {
            let contenido = "ERROR: There is no bomb at position provided".to_string();
            escribir_archivo(dir_output, contenido)
        }
    } else if let Err(err) = Mapa::new(map_reading) {
        escribir_archivo(dir_output, err);
    }

    //
}
