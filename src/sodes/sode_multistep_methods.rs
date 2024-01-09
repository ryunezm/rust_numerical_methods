// ------ TRAIT ------

pub(crate) trait Functions {
    fn info();
    fn new(function: Box<dyn Fn(f64, f64) -> f64>, a: f64, b: f64, n: f64) -> Self;
    fn solve();
}

// ------ STRUCTS ------

pub struct Trapezoidal {}

pub struct AdamsBashforth {}

pub struct AdamsMoulton {}
// ------ IMPLEMENTATIONS ------

impl Functions for Trapezoidal {
    fn info() {
        println!("Welcome to Trapezoidal Method");
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, a: f64, b: f64, n: f64) -> Self {
        todo!()
    }

    fn solve() {
        todo!()
    }
}

impl Functions for AdamsBashforth {
    fn info() {
        println!("Welcome to Adams-Bashforth Method");
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, a: f64, b: f64, n: f64) -> Self {
        todo!()
    }

    fn solve() {
        todo!()
    }
}

impl Functions for AdamsMoulton {
    fn info() {
        println!("Welcome to Adams-Moulton Method");
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, a: f64, b: f64, n: f64) -> Self {
        todo!()
    }

    fn solve() {
        todo!()
    }
}


//println!("7. Trapezoidal method");
//println!("8. Adams-Bashforth method");
//println!("9. Adams-Moulton method");