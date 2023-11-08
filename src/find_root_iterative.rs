// ------ TRAIT ------
trait FindRoot {
    fn solve(&mut self) -> f64;
}
// ------ STRUCTS ------

pub struct Newton { function: fn(f64) -> f64, function_derivative: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize }

pub struct Secant { function: fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64, max_iter: usize }

pub struct Steffensen { function: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize }

// ------ IMPLEMENTATIONS ------

impl Newton {
    fn new(function: fn(f64) -> f64, function_derivative: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize)
        -> Self { Newton {function, function_derivative, x0, tolerance, max_iter} }
}

impl FindRoot for Newton {
    fn solve(&mut self) -> f64 {
        todo!()
    }
}

impl Secant {
    fn new(function: fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64, max_iter: usize)
        -> Self { Secant {function, x0, x1, tolerance, max_iter} }
}