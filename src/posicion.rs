#[derive(Clone, Debug)]
pub struct Posicion {
    pub x: usize,
    pub y: usize,
}

impl Posicion {
    pub fn new(a: usize, b: usize) -> Self {
        Self { x: a, y: b }
    }

    pub fn sumar(&self, punto: (usize, usize)) -> Self {
        Self { x: self.x + punto.0, y: self.y + punto.1}
    }

    pub fn check_resta(&self, punto: (usize, usize)) -> Option<Self> {
        if punto.0 <= self.x && punto.1 <= self.y {
            Some( Self { x: self.x - punto.0, y: self.y - punto.1} )
        }
        else { None }
    }
}