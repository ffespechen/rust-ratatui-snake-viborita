use crate::models::{Direccion, Posicion};
use rand::{self, RngExt};
use ratatui::style::Color;
use std::collections::VecDeque;

pub struct Juego {
    pub serpiente: VecDeque<Posicion>,
    pub direccion_serpiente: Direccion,
    pub posicion_comida: Posicion,
    pub color_comida: Color,
    pub puntaje: i32,
    pub exit: bool,
}

const COLORES_COMIDA: [Color; 6] = [
    Color::Red,
    Color::Cyan,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];

impl Juego {
    pub fn new(ancho: u16, alto: u16) -> Self {
        let posicion_inicial = Posicion {
            x: ancho / 2,
            y: alto / 2,
        };
        let posicion_comida = Posicion {
            x: rand::rng().random_range(0..ancho),
            y: rand::rng().random_range(0..alto),
        };
        let mut serpiente = VecDeque::new();
        serpiente.push_back(posicion_inicial);
        Juego {
            serpiente,
            direccion_serpiente: Direccion::Abajo,
            posicion_comida,
            color_comida: Color::Red,
            puntaje: 0,
            exit: false,
        }
    }

    fn generar_posicion_aleatoria(&self, ancho: u16, alto: u16) -> Posicion {
        let x = rand::rng().random_range(0..ancho);
        let y = rand::rng().random_range(0..alto);
        Posicion { x, y }
    }

    fn calcular_siguiente_posicion(&self, ancho: u16, alto: u16) -> Posicion {
        let mut posicion = self.serpiente.front().unwrap().clone();
        match self.direccion_serpiente {
            Direccion::Abajo => {
                posicion.y = posicion.y.saturating_sub(1);
            }
            Direccion::Arriba => {
                if (posicion.y + 1) < alto {
                    posicion.y = posicion.y + 1;
                }
            }
            Direccion::Derecha => {
                if (posicion.x + 1) < ancho {
                    posicion.x = posicion.x + 1;
                }
            }
            Direccion::Izquierda => {
                posicion.x = posicion.x.saturating_sub(1);
            }
        }
        posicion
    }

    pub fn mover(&mut self, ancho: u16, alto: u16) {
        let nueva_cabeza = self.calcular_siguiente_posicion(ancho, alto);

        if nueva_cabeza == *self.serpiente.front().unwrap() {
            self.exit = true; // Colisión con la cabeza, termina el juego
            return; // No mover si la nueva posición es la misma que la cabeza actual
        }

        self.serpiente.push_front(nueva_cabeza);

        if self.posicion_comida == nueva_cabeza {
            let mut nueva_posicion_comida = self.generar_posicion_aleatoria(ancho, alto);
            while self.serpiente.contains(&nueva_posicion_comida) {
                nueva_posicion_comida = self.generar_posicion_aleatoria(ancho, alto);
            }
            self.posicion_comida = nueva_posicion_comida;
            self.color_comida = COLORES_COMIDA[rand::rng().random_range(0..COLORES_COMIDA.len())];
            self.puntaje += 10;
        } else {
            let _ = self.serpiente.pop_back();
        }
    }

    pub fn verificar_colisiones(&mut self, ancho: u16, alto: u16) {
        let cabeza = self.serpiente.front().unwrap();

        // Colisiones con paredes
        if cabeza.x >= ancho || cabeza.y >= alto {
            self.exit = true;
        }

        // Colisiones con el cuerpo
        if self.serpiente.iter().skip(1).any(|pos| pos == cabeza) {
            self.exit = true;
        }
    }
}
