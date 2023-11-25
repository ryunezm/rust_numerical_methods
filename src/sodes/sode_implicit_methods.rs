// ------ TRAIT ------
pub(crate)  trait Functions{
    fn info();
    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self;
    fn solve(&mut self) -> Option<(f64)>;
}

// ------ STRUCTS ------

pub struct ImplicitEuler {
    function: Box<dyn Fn(f64, f64) -> f64>,
    y0: f64,
    h: f64,
    n: f64,
}

pub struct BackwardEuler {
    function: Box<dyn Fn(f64, f64) -> f64>,
    y0: f64,
    h: f64,
    n: f64,
}

pub struct CrankNicolson {
    function: Box<dyn Fn(f64, f64) -> f64>,
    y0: f64,
    h: f64,
    n: f64,

}

// ------ IMPLEMENTATIONS ------

impl Functions for ImplicitEuler {
    fn info() {
        todo!()
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self {
        ImplicitEuler { function, y0, h, n }
    }

    fn solve(&mut self) -> Option<(f64)> {
        todo!()
    }
}

impl Functions for BackwardEuler {
    fn info() {
        todo!()
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self {
        BackwardEuler { function, y0, h, n }
    }

    fn solve(&mut self) -> Option<(f64)> {
        todo!()
    }
}

impl Functions for CrankNicolson {
    fn info() {
        todo!()
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self {
        CrankNicolson { function, y0, h, n }
    }

    fn solve(&mut self) -> Option<(f64)> {
        todo!()
    }
}