use std::ops::Index;
use std::ops::IndexMut;

fn main() {
    use Celda::*;
    let tablero: Tablero =
        [ Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        ].into();
    println!("{:?}", siguiente_jugada(tablero, Jugador::Cruz));
}

fn mejor_puntuacion(tablero: &Tablero, jugador: Jugador) -> (Accion, i32) {
    tablero
        .acciones(jugador)
        .iter()
        .map(|&x| (x, tablero.efectuar_accion(x).puntuacion(jugador)))
        .max_by(|&(_, x), &(_, y)| x.cmp(&y))
        .unwrap()
}

fn siguiente_jugada(tablero: Tablero, jugador: Jugador) -> Accion {
    let p1 = mejor_puntuacion(&tablero, jugador);
    let p2 = mejor_puntuacion(&tablero, jugador.opuesto());
    if p1.1 >= p2.1 {
        p1.0
    } else {
        match p2.0 {
            Accion::Cruz(i) => Accion::Circulo(i),
            Accion::Circulo(i) => Accion::Cruz(i),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Jugador {
    Cruz,
    Circulo,
}

impl Jugador {
    fn opuesto(self) -> Jugador {
        match self {
            Jugador::Cruz => Jugador::Circulo,
            Jugador::Circulo => Jugador::Cruz,
        }
    }
}

impl Into<Celda> for Jugador {
    fn into(self) -> Celda {
        match self {
            Jugador::Cruz => Celda::Cruz,
            Jugador::Circulo => Celda::Circulo,
        }
    }
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

static lineas: [[usize; 3]; 8] =
    [ [0, 3, 6]
    , [1, 4, 7]
    , [2, 5, 8]
    , [0, 1, 2]
    , [3, 4, 5]
    , [6, 7, 8]
    , [0, 4, 8]
    , [2, 4, 6]
    ];

impl Tablero {
    fn puntuacion(&self, jugador: Jugador) -> i32 {
        if self.es_fin_de_partida() { return 8*4 }
        let mut puntuacion = 0;
        for l in lineas.iter() {
            let mut p = 0;
            for &i in l.iter() {
                if self[i] == jugador.into() {
                    p += 1;
                } else if self[i] != Celda::Vacia {
                    p = -1;
                    break;
                }
            }
            puntuacion += p + 1;
        }
        puntuacion
    }

    fn dar_la_vuelta(&self) -> Tablero {
        let t =
            self
                .0
                .iter()
                .map(|&celda| {
                    match celda {
                        Celda::Cruz => Celda::Circulo,
                        Celda::Circulo => Celda::Cruz,
                        Celda::Vacia => Celda::Vacia,
                    }
                })
                .collect::<Vec<Celda>>();
        Tablero([t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7], t[8]])
    }

    fn es_fin_de_partida(&self) -> bool {
        use Celda::*;
           (self[0] != Vacia && self[0] == self[1] && self[1] == self[2])
        || (self[3] != Vacia && self[3] == self[4] && self[4] == self[5])
        || (self[6] != Vacia && self[6] == self[7] && self[7] == self[8])

        || (self[0] != Vacia && self[0] == self[3] && self[3] == self[6])
        || (self[1] != Vacia && self[1] == self[4] && self[4] == self[7])
        || (self[2] != Vacia && self[2] == self[5] && self[5] == self[8])

        || (self[0] != Vacia && self[0] == self[4] && self[4] == self[8])
        || (self[2] != Vacia && self[2] == self[4] && self[4] == self[6])
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
                if self[i] != Celda::Vacia { panic!("¡Ahí no se puede pintar!") }
                nuevo[i] = Celda::Cruz;
            }
            Accion::Circulo(i) => {
                if self[i] != Celda::Vacia { panic!("¡Ahí no se puede pintar!") }
                nuevo[i] = Celda::Circulo;
            }
        }
        nuevo
    }
}
