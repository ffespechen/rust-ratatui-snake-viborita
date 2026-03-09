#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Posicion {
    pub x: u16,
    pub y: u16,
}
