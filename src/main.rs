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
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::env;

fn main() {
    // Catch the arguments sent via the command line
    let args: Vec<String> = env::args().collect();

    // Create a path to the desired input file
    let path = Path::new(&args[1]);

    // Create a path to the desired output file
    let dir_output = Path::new(&args[2]);

    let ter_arg = args[3].parse::<usize>();
    let cuart_arg = args[4].parse::<usize>();
    let mut dir_x = 0;
    let mut dir_y = 0;

    match ter_arg {
        Err(why) => println!("{}", why),
        Ok(resultado) => dir_x = resultado,
    }

    match cuart_arg {
        Err(why) => println!("{}", why),
        Ok(resultado) => dir_y = resultado,
    }

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

    // Create the map from the contents of the file, active the bomb in the coordinates provided
    // and generated the output in a string
    let mut mapa = Mapa::new(s);
    mapa.herir_objeto(Posicion { x: dir_y, y: dir_x }, "_00".to_string());
    let output = mapa.mostrar();

    // Create the output file
    let display = dir_output.display();
    let mut file_output = match File::create(dir_output) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file_output) => file_output,
    };

    // Write the output file if possible with the output map
    match file_output.write_all(output.as_bytes()) {
        Err(why) => println!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
