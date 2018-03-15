use std::ops::Index;
use std::ops::IndexMut;

fn main() {
    use Celda::*;
    let tablero: Tablero =
        [ Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        ].into();
    println!("{:?}", mejor_jugada(&tablero, Jugador::Cruz));
}

fn mejor_jugada(tablero: &Tablero, jugador: Jugador) -> Accion {
    tablero
        .acciones(jugador)
        .iter()
        .map(|&accion| (accion, tablero.efectuar_accion(accion).puntuacion(jugador)))
        .inspect(|x| print!("{:?} ", x))
        .max_by(|&(_, x), &(_, y)| x.cmp(&y))
        .unwrap()
        .0
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Jugador {
    Cruz,
    Circulo,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Celda {
    Vacia,
    Cruz,
    Circulo,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Tablero([Celda; 9]);

impl Default for Tablero {
    fn default() -> Tablero {
        use Celda::*;
        [ Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        ].into()
    }
}

impl Index<usize> for Tablero {
    type Output = Celda;
    fn index(&self, i: usize) -> &Celda {
        &self.0[i]
    }
}

impl IndexMut<usize> for Tablero {
    fn index_mut(&mut self, i: usize) -> &mut Celda {
        &mut self.0[i]
    }
}

impl From<[Celda; 9]> for Tablero {
    fn from(arr: [Celda; 9]) -> Tablero {
        Tablero(arr)
    }
}

#[derive(Clone, Copy, Debug)]
enum Accion {
    Cruz(usize),
    Circulo(usize),
}

impl Tablero {
    fn puntuacion(&self, jugador: Jugador) -> i32 {
        match self.ganador() {
            Some(j) if j == jugador => return 1,
            Some(_)                 => return -1,
            None                    => (),
        }

        self.acciones(self.turno())
            .iter()
            .map(|&accion| self.efectuar_accion(accion).puntuacion(jugador))
            .sum()
    }

    fn ganador(&self) -> Option<Jugador> {
        use Celda::*;
        if (self[0] == Cruz && self[0] == self[1] && self[1] == self[2])
        || (self[3] == Cruz && self[3] == self[4] && self[4] == self[5])
        || (self[6] == Cruz && self[6] == self[7] && self[7] == self[8])

        || (self[0] == Cruz && self[0] == self[3] && self[3] == self[6])
        || (self[1] == Cruz && self[1] == self[4] && self[4] == self[7])
        || (self[2] == Cruz && self[2] == self[5] && self[5] == self[8])

        || (self[0] == Cruz && self[0] == self[4] && self[4] == self[8])
        || (self[2] == Cruz && self[2] == self[4] && self[4] == self[6]) {
            Some(Jugador::Cruz)
        } else if
               (self[0] == Circulo && self[0] == self[1] && self[1] == self[2])
            || (self[3] == Circulo && self[3] == self[4] && self[4] == self[5])
            || (self[6] == Circulo && self[6] == self[7] && self[7] == self[8])

            || (self[0] == Circulo && self[0] == self[3] && self[3] == self[6])
            || (self[1] == Circulo && self[1] == self[4] && self[4] == self[7])
            || (self[2] == Circulo && self[2] == self[5] && self[5] == self[8])

            || (self[0] == Circulo && self[0] == self[4] && self[4] == self[8])
            || (self[2] == Circulo && self[2] == self[4] && self[4] == self[6]) {
            Some(Jugador::Circulo)
        } else {
            None
        }
    }

    fn acciones(&self, jugador: Jugador) -> Vec<Accion> {
        let mut vec = Vec::new();
        let mut i = 0;
        for &x in self.0.iter() {
            if x == Celda::Vacia {
                vec.push(match jugador {
                    Jugador::Cruz => Accion::Cruz(i),
                    Jugador::Circulo => Accion::Circulo(i),
                });
            }
            i += 1;
        }
        vec
    }

    fn turno(&self) -> Jugador {
        let mut i = 0;
        for &x in self.0.iter() {
            match x {
                Celda::Vacia => (),
                Celda::Cruz => i += 1,
                Celda::Circulo => i -= 1,
            }
        }
        match i {
            // mismo número de cruces y círculos o hay un círculo más
            0 | -1 => Jugador::Cruz,
            1 => Jugador::Circulo,
            _ => panic!("¡Alguien ha hecho trampas!"),
        }
    }

    fn efectuar_accion(&self, accion: Accion) -> Tablero {
        let mut nuevo = self.clone();
        match accion {
            Accion::Cruz(i) => {
                if self[i] != Celda::Vacia { panic!("¡Ahí no se puede jugar!") }
                nuevo[i] = Celda::Cruz;
            }
            Accion::Circulo(i) => {
                if self[i] != Celda::Vacia { panic!("¡Ahí no se puede jugar!") }
                nuevo[i] = Celda::Circulo;
            }
        }
        nuevo
    }
}
