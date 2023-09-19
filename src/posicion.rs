#[derive(Clone, Debug)]
 
/// It is the definition of the position type
/// 
/// # What is inside
/// The 'x' variable indicates the number of file
/// ```
/// pub x: usize,
/// ```
/// The 'y' variable indicates the number of column
/// ```
/// pub y: usize,
/// ```
pub struct Posicion {
    pub x: usize,
    pub y: usize,
}

impl Posicion {
    ///
    /// Creates a position instance from the coordinates provided
    pub fn new(a: usize, b: usize) -> Self {
        Self { x: a, y: b }
    }

    ///
    /// Implements the logic of adding a position to the current position
    pub fn sumar(&self, punto: (usize, usize)) -> Self {
        Self {
            x: self.x + punto.0,
            y: self.y + punto.1,
        }
    }

    ///
    /// Implements the logic of removing a position to the current position
    /// It returns an Option in case the resulting point is negative on some direction
    pub fn check_resta(&self, punto: (usize, usize)) -> Option<Self> {
        if punto.0 <= self.x && punto.1 <= self.y {
            Some(Self {
                x: self.x - punto.0,
                y: self.y - punto.1,
            })
        } else {
            None
        }
    }
}
