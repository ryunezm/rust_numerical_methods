// ------ TRAIT ------
pub(crate) trait FindRoot {
    fn info();
    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64, max_iter: usize) -> Self;
    fn solve(&mut self) -> (f64, usize);
}
// ------ STRUCTS ------

pub struct Brent{
    function: Box<dyn Fn(f64) -> f64>,
    a: f64,
    b: f64,
    tolerance: f64,
    max_iter: usize,
}

pub struct Ridder{
    function: Box<dyn Fn(f64) -> f64>,
    a: f64,
    b: f64,
    tolerance: f64,
    max_iter: usize,
}

// ------ IMPLEMENTATIONS ------

impl FindRoot for Brent{
    fn info() {
        println!("Welcome to Brent's Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. An interval (a, b) in which the function will be evaluated in search of the roots.");
        println!("3. Tolerance (tol).");
        println!("4. Maximum number of iterations");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64, max_iter: usize) -> Self {
        Brent{
            function,
            a,
            b,
            tolerance,
            max_iter,
        }
    }

    fn solve(&mut self) -> (f64, usize) {
        todo!()
    }
}

impl FindRoot for Ridder {
    fn info() {
        println!("Welcome to Ridder's Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. An interval (a, b) in which the function will be evaluated in search of the roots.");
        println!("3. Tolerance (tol).");
        println!("4. Maximum number of iterations");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64, max_iter: usize) -> Self {
        Ridder{
            function,
            a,
            b,
            tolerance,
            max_iter,
        }
    }

    fn solve(&mut self) -> (f64, usize) {
        todo!()
    }
}

