use std::ops::Index;
use std::ops::IndexMut;

fn main() {
    use Celda::*;
    let tablero: Tablero =
        [ Vacia, Vacia, Circulo
        , Cruz, Cruz , Vacia
        , Cruz , Vacia, Circulo
        ].into();
    //println!("{}", tablero.puntuacion(Circulo));
    println!(
        "{:?}",
        tablero
            .acciones()
            .iter()
            .map(|&x| (x, tablero.turno(x).puntuacion(Circulo)))
            .max_by(|&(_, x), &(_, y)| x.cmp(&y))
            .unwrap()
            .0
    );
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
    fn puntuacion(&self, jugador: Celda) -> i32 {
        use Celda::*;
        assert!(jugador != Vacia);
        let mut puntuacion = 0;
        for l in lineas.iter() {
            let mut p = 0;
            for &i in l.iter() {
                if self[i] == jugador {
                    p += 1;
                } else if self[i] != Vacia {
                    p = -1;
                    break;
                }
            }
            puntuacion += p + 1;
        }
        puntuacion
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

    fn ganador(&self) -> Celda {
        use Celda::*;
        if (self[0] == Cruz && self[0] == self[1] && self[1] == self[2])
        || (self[3] == Cruz && self[3] == self[4] && self[4] == self[5])
        || (self[6] == Cruz && self[6] == self[7] && self[7] == self[8])

        || (self[0] == Cruz && self[0] == self[3] && self[3] == self[6])
        || (self[1] == Cruz && self[1] == self[4] && self[4] == self[7])
        || (self[2] == Cruz && self[2] == self[5] && self[5] == self[8])

        || (self[0] == Cruz && self[0] == self[4] && self[4] == self[8])
        || (self[2] == Cruz && self[2] == self[4] && self[4] == self[6]) {
            Cruz
        } else if
               (self[0] == Circulo && self[0] == self[1] && self[1] == self[2])
            || (self[3] == Circulo && self[3] == self[4] && self[4] == self[5])
            || (self[6] == Circulo && self[6] == self[7] && self[7] == self[8])

            || (self[0] == Circulo && self[0] == self[3] && self[3] == self[6])
            || (self[1] == Circulo && self[1] == self[4] && self[4] == self[7])
            || (self[2] == Circulo && self[2] == self[5] && self[5] == self[8])

            || (self[0] == Circulo && self[0] == self[4] && self[4] == self[8])
            || (self[2] == Circulo && self[2] == self[4] && self[4] == self[6]) {
            Circulo
        } else {
            Vacia
        }
    }

    fn acciones(&self) -> Vec<Accion> {
        let mut i = 0;
        for &x in self.0.iter() {
            match x {
                Celda::Vacia => (),
                Celda::Cruz => i += 1,
                Celda::Circulo => i -= 1,
            }
        }
        match i {
            0 | -1 => {  // mismo número de cruces y círculos o hay un círculo más
                let mut vec = Vec::new();
                let mut i = 0;
                for &x in self.0.iter() {
                    if x == Celda::Vacia {
                        vec.push(Accion::Cruz(i));
                    }
                    i += 1;
                }
                vec
            },
            1 => {
                let mut vec = Vec::new();
                let mut i = 0;
                for &x in self.0.iter() {
                    if x == Celda::Vacia {
                        vec.push(Accion::Circulo(i));
                    }
                    i += 1;
                }
                vec
            },
            _ => panic!("¡Alguien ha hecho trampas!"),
        }
    }

    fn turno(&self, accion: Accion) -> Tablero {
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
