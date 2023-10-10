mod bomba_normal;
mod casillero;
mod desvio;
mod enemigo;
mod mapa;
mod pared;
mod posicion;
mod rafaga;
mod roca;
mod vacio;

use crate::mapa::Mapa;

use crate::posicion::Posicion;
use crate::rafaga::Rafaga;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub fn escribir_archivo(path: &Path, contenido: String) {
    // Create the output file
    let display = path.display();

    if let Ok(file_output) = File::create(path) {
        let mut file = file_output;
        if let Err(why) = file.write_all(contenido.as_bytes()) {
            println!("couldn't write to {}: {}", display, why)
        }
    }
}

pub fn activar_primer_bomba(maps: Mapa, dir_y: usize, dir_x: usize, dir_output: &Path) {
    let mut mapa = maps;
    if mapa.there_is_a_bomb_at(Posicion { x: dir_y, y: dir_x }) {
        mapa.herir_objeto(Rafaga::new(
            "_".to_string(),
            2,
            Posicion { x: dir_y, y: dir_x },
        ));
        escribir_archivo(dir_output, mapa.mostrar())
    } else {
        escribir_archivo(
            dir_output,
            format!("There is not a bomb at {} {}", dir_y, dir_x),
        )
    }
}

fn main() {
    // Catch the arguments sent via the command line
    let args: Vec<String> = env::args().collect();

    // Create a path to the desired input file
    let path = Path::new(&args[1]);

    // Create a path to the desired output file
    let mut file_output = PathBuf::new();
    file_output.push(&args[2]);

    match path.file_name() {
        Some(p) => file_output.push(p),
        _ => {
            println!("Could not create output file");
            return;
        }
    }

    let dir_x: usize;
    let dir_y: usize;

    let map_reading: String;

    if let Ok(ter_arg) = args[3].parse::<usize>() {
        dir_x = ter_arg;
    } else {
        let contenido = "ERROR: Could not parse first argument".to_string();
        escribir_archivo(&file_output, contenido);
        return;
    }

    if let Ok(cuart_arg) = args[4].parse::<usize>() {
        dir_y = cuart_arg;
    } else {
        let contenido = "ERROR: Could not parse second argument".to_string();
        escribir_archivo(&file_output, contenido);
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
            escribir_archivo(&file_output, contenido);
            return;
        }
    } else {
        let contenido = "ERROR: Could not open file".to_string();
        escribir_archivo(&file_output, contenido);
        return;
    }

    // Create the map from the contents of the file, active the bomb in the coordinates provided
    // and generated the output in a string
    match Mapa::new(map_reading) {
        Ok(maps) => activar_primer_bomba(maps, dir_y, dir_x, &file_output),
        Err(e) => escribir_archivo(&file_output, e),
    }
}
