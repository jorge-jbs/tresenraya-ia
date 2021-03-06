use std::rc::Rc;
use std::ops::Index;
use std::ops::IndexMut;
use std::collections::hash_set::HashSet;

fn main() {
    use Celda::*;
    /*
    println!("{:?}", breadth_first_search(
        [ Vacia, Circulo, Vacia
        , Vacia, Cruz   , Vacia
        , Vacia, Vacia  , Cruz
        ].into(),
        Cruz,
    ));
    */
    /*
    println!("{:?}", breadth_first_search(
        [ Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        , Vacia, Vacia, Vacia
        ].into(),
        Circulo
    ));
    */
    println!("{:?}", breadth_first_search(
        [ Cruz , Vacia  , Cruz
        , Vacia, Vacia  , Vacia
        , Vacia, Circulo, Vacia
        ].into(),
        Cruz
    ));
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

#[derive(Clone, Debug)]
struct Nodo {
    padre: Option<Rc<Nodo>>,
    accion: Option<Accion>,
    tablero: Tablero,
    coste: i32,
}

impl Tablero {
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

fn breadth_first_search(tablero_inicial: Tablero, ganador: Celda) -> Vec<Accion> {
    let nodo = Nodo {
        padre: None,
        accion: None,
        tablero: tablero_inicial,
        coste: 0,
    };
    if nodo.tablero.es_fin_de_partida() { return vec![] }
    let mut frontera = vec![nodo];
    let mut explorado: HashSet<Tablero> = HashSet::new();
    loop {
        //print!("{} ", frontera.len());
        println!("{:?}", frontera.iter().filter(|x| x.padre.is_some()).map(|x| Rc::strong_count(&x.clone().padre.unwrap())).collect::<Vec<_>>());

        //println!("{:?}\n\n", frontera.iter().map(|x| x.accion).collect::<Vec<_>>());
        //println!("{:?}\n\n", frontera);
        if frontera.is_empty() { panic!("Fallé. ¡Imposible!") }
        let nodo = frontera.pop().unwrap();
        explorado.insert(nodo.tablero.clone());
        for accion in nodo.tablero.acciones() {
            let hijo = nodo_hijo(nodo.clone(), accion);
            if !explorado.contains(&hijo.tablero) && frontera.iter().find(|x| x.tablero == hijo.tablero).is_none() {
                if hijo.tablero.ganador() == ganador {
                    return recopilar_acciones(hijo)
                }
                frontera.push(hijo);
            }
        }
    }
}

fn nodo_hijo(padre: Nodo, accion: Accion) -> Nodo {
    Nodo {
        accion: Some(accion),
        tablero: padre.tablero.turno(accion),
        coste: padre.coste + 1,
        padre: Some(Rc::new(padre)),
    }
}

fn recopilar_acciones(nodo_inicial: Nodo) -> Vec<Accion> {
    let mut acciones = Vec::new();
    let mut nodo = Rc::new(nodo_inicial);
    loop {
        //println!("--{:?}", nodo);
        if nodo.accion.is_some() {
            //acciones.push(nodo.accion.unwrap());
            acciones.insert(0, nodo.accion.unwrap());
        }
        match nodo.padre.clone() {
            Some(ref padre) => nodo = Rc::clone(padre),
            None => {
                println!("Terminé!!");
                return acciones
            },
        }
    }
}
