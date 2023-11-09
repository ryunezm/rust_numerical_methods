// ------ TRAIT ------
trait FindRoot {
    fn solve(&mut self) -> f64;
}
// ------ STRUCTS ------

pub struct Newton
{ function: fn(f64) -> f64, function_derivative: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize }

pub struct Secant
{ function: fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64, max_iter: usize }

pub struct Steffensen
{ function: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize }

// ------ IMPLEMENTATIONS ------

impl Newton {
    fn new(function: fn(f64) -> f64, function_derivative: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize)
           -> Self { Newton { function, function_derivative, x0, tolerance, max_iter } }
}

impl FindRoot for Newton {
    fn solve(&mut self) -> f64 {
        let mut x = self.x0;
        let mut iter = 0;

        while iter < self.max_iter {
            let fx = (self.function)(x);
            let fx_derivative = (self.function_derivative)(x);
            if fx_derivative.abs() < 1e-10 { break; } // Division by 0 is avoided here.

            let x_next = x - fx / fx_derivative;
            if (x_next - x).abs() < self.tolerance { return x_next; }

            x = x_next;
            iter += 1;
        }

        x
    }
}

impl Secant {
    fn new(function: fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64, max_iter: usize)
           -> Self { Secant { function, x0, x1, tolerance, max_iter } }
}

impl FindRoot for Secant {
    fn solve(&mut self) -> f64 {
        let mut x_prev = self.x0;
        let mut x = self.x1;
        let mut iter = 0;

        while iter < self.max_iter {
            let fx = (self.function)(x);
            let fx_prev = (self.function)(x_prev);

            if (fx - fx_prev).abs() < self.tolerance { return x; }

            let x_next = x - fx * (x - x_prev) / (fx - fx_prev);
            x_prev = x;
            x = x_next;
            iter += 1;
        }

        x
    }
}

impl Steffensen {
    fn new(function: fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize)
           -> Self {Steffensen { function, x0, tolerance, max_iter }}
}

impl FindRoot for Steffensen {
    fn solve(&mut self) -> f64 {
        let mut x = self.x0;
        let mut iter = 0;

        while iter < self.max_iter {
            let fx = (self.function)(x);
            let fxx = (self.function)(x+fx) - fx;
            let x_next = x - fx*fx/fxx;

            if (x_next-x).abs() < self.tolerance { return x_next }

            x = x_next;

            iter+=1;
        }
        x
    }
}