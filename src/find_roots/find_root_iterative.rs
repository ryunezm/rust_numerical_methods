// ------ TRAIT ------
pub(crate) trait FindRoot {
    fn info();
    fn solve(&mut self) -> (f64, usize);
}

// ------ STRUCTS ------

pub struct Newton {
    function: Box<dyn Fn(f64) -> f64>,
    function_derivative: fn(f64) -> f64,
    x0: f64,
    tolerance: f64,
    max_iter: usize,
}

pub struct Secant {
    function: Box<dyn Fn(f64) -> f64>,
    x0: f64,
    x1: f64,
    tolerance: f64,
    max_iter: usize,
}

pub struct Steffensen {
    function: Box<dyn Fn(f64) -> f64>,
    x0: f64,
    tolerance: f64,
    max_iter: usize,
}

pub struct FixedPoint {
    function: Box<dyn Fn(f64) -> f64>,
    x0: f64,
    tolerance: f64,
    max_iter: usize,
}

pub struct InverseQuadraticInterpolation {
    function: Box<dyn Fn(f64) -> f64>,
    x0: f64,
    x1: f64,
    x2: f64,
    tolerance: f64,
    max_iter: usize,
}

// ------ IMPLEMENTATIONS ------

impl Newton {
    fn new(
        function: Box<dyn Fn(f64) -> f64>,
        function_derivative: fn(f64) -> f64,
        x0: f64,
        tolerance: f64,
        max_iter: usize,
    ) -> Self {
        Newton {
            function,
            function_derivative,
            x0,
            tolerance,
            max_iter,
        }
    }
}

impl FindRoot for Newton {
    fn info() {
        println!("Welcome to Newton Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. Derivative of the function (f')");
        println!("3. A initial value (x0).");
        println!("4. Tolerance (tol).");
        println!("5. Maximum number of iterations (max_iter).");
        println!("Please enter the data:")
    }

    fn solve(&mut self) -> (f64, usize) {
        let mut x = self.x0;
        let mut iter = 0;

        while iter < self.max_iter {
            let fx = (self.function)(x);
            let fx_derivative = (self.function_derivative)(x);
            if fx_derivative.abs() < 1e-10 {
                break;
            } // Division by 0 is avoided here.

            let x_next = x - fx / fx_derivative;
            if (x_next - x).abs() < self.tolerance {
                return (x_next, iter);
            }

            x = x_next;
            iter += 1;
        }

        (x, iter)
    }
}

impl Secant {
    fn new(function: Box<dyn Fn(f64) -> f64>, x0: f64, x1: f64, tolerance: f64, max_iter: usize) -> Self {
        Secant {
            function,
            x0,
            x1,
            tolerance,
            max_iter,
        }
    }
}

impl FindRoot for Secant {
    fn info() {
        println!("Welcome to Secant Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. Two initial values (x0, x1). They should be close to the desired zero.");
        println!("3. Tolerance (tol).");
        println!("4. Maximum number of iterations (max_iter).");
        println!("Please enter the data:")
    }

    fn solve(&mut self) -> (f64, usize) {
        let mut x_prev = self.x0;
        let mut x = self.x1;
        let mut iter = 0;

        while iter < self.max_iter {
            let fx = (self.function)(x);
            let fx_prev = (self.function)(x_prev);

            if (fx - fx_prev).abs() < self.tolerance {
                return (x, iter);
            }

            let x_next = x - fx * (x - x_prev) / (fx - fx_prev);
            x_prev = x;
            x = x_next;
            iter += 1;
        }

        (x, iter)
    }
}

impl Steffensen {
    fn new(function: Box<dyn Fn(f64) -> f64>, x0: f64, tolerance: f64, max_iter: usize) -> Self {
        Steffensen {
            function,
            x0,
            tolerance,
            max_iter,
        }
    }
}

impl FindRoot for Steffensen {
    fn info() {
        println!("Welcome to Steffensen's Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. A initial value (x0).");
        println!("3. Tolerance (tol).");
        println!("4. Maximum number of iterations (max_iter).");
        println!("Please enter the data:")
    }

    fn solve(&mut self) -> (f64, usize) {
        let mut x = self.x0;
        let mut iter = 0;

        while iter < self.max_iter {
            let fx = (self.function)(x);
            let fxx = (self.function)(x + fx) - fx;
            let x_next = x - fx * fx / fxx;

            if (x_next - x).abs() < self.tolerance {
                return (x_next, iter);
            }

            x = x_next;

            iter += 1;
        }
        (x, iter)
    }
}

impl FixedPoint {
    fn new(function: Box<dyn Fn(f64) -> f64>, x0: f64, tolerance: f64, max_iter: usize) -> Self {
        FixedPoint {
            function,
            x0,
            tolerance,
            max_iter,
        }
    }
}

impl FindRoot for FixedPoint {
    fn info() {
        println!("Welcome to Fixed Point Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. A initial value (x0).");
        println!("3. Tolerance (tol).");
        println!("4. Maximum number of iterations (max_iter).");
        println!("Please enter the data:")
    }

    fn solve(&mut self) -> (f64, usize) {
        let mut iter = 0;
        let mut x0 = self.x0;

        while iter < self.max_iter {
            let x_next = (self.function)(x0);

            if (x_next - x0).abs() < self.tolerance {
                return (x_next, iter);
            }

            x0 = x_next;
            iter += 1;
        }

        (x0, iter)
    }
}

impl InverseQuadraticInterpolation {
    fn new(
        function: Box<dyn Fn(f64) -> f64>,
        x0: f64,
        x1: f64,
        x2: f64,
        tolerance: f64,
        max_iter: usize,
    ) -> Self {
        InverseQuadraticInterpolation {
            function,
            x0,
            x1,
            x2,
            tolerance,
            max_iter,
        }
    }
}

impl FindRoot for InverseQuadraticInterpolation {
    fn info() {
        println!("Welcome to Inverse Quadratic Interpolation Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. Three initial values (x0, x1, x2).");
        println!("3. Tolerance (tol).");
        println!("4. Maximum number of iterations (max_iter).");
        println!("Please enter the data:")
    }

    fn solve(&mut self) -> (f64, usize) {
        let mut iter = 0;
        let mut x0 = self.x0;
        let mut x1 = self.x1;
        let mut x2 = self.x2;

        while iter < self.max_iter {
            let f0 = (self.function)(self.x0);
            let f1 = (self.function)(self.x1);
            let f2 = (self.function)(self.x2);

            let a = f0 / ((f1 - f0) * (x2 - x0) - (f2 - f0) * (x1 - x0));
            let b = f1 / ((f0 - f1) * (x2 - x1) - (f2 - f1) * (x0 - x1));
            let c = f2 / ((f0 - f2) * (x1 - x2) - (f1 - f2) * (x0 - x2));

            let x_next = x0 * a + x1 * b + x2 * c;

            if (x_next - x1).abs() < self.tolerance {
                return (x_next, iter);
            }

            x0 = x1;
            x1 = x2;
            x2 = x_next;
            iter += 1;
        }

        (x2, iter)
    }
}
