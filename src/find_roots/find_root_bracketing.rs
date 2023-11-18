// ------ TRAIT ------
pub(crate) trait FindRoot {
    fn info();
    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64) -> Self;
    fn solve(&mut self) -> (f64, f64, usize);
}

// ------ STRUCTS ------
pub struct Bisection {
    function: Box<dyn Fn(f64) -> f64>,
    a: f64,
    b: f64,
    tolerance: f64,
}

pub struct FalsePosition {
    function: Box<dyn Fn(f64) -> f64>,
    a: f64,
    b: f64,
    tolerance: f64,
}

pub struct ITP {
    function: Box<dyn Fn(f64) -> f64>,
    a: f64,
    b: f64,
    tolerance: f64,
}

// ------ IMPLEMENTATIONS ------
impl FindRoot for Bisection {
    fn info() {
        println!("Welcome to Bisection Method");
        println!("You  need to provide:");
        println!("1. A function (f).");
        println!("2. An interval (a, b) in which the function will be evaluated in search of the roots.");
        println!("3. Tolerance (tol).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64) -> Self {
        Bisection {
            function,
            a,
            b,
            tolerance,
        }
    }

    fn solve(&mut self) -> (f64, f64, usize) {
        let max_iter = f64::log2((self.a - self.b) / self.tolerance).ceil() as usize;
        let mut iter = 0;

        let mut error_abs = (self.b - self.a).abs();

        while error_abs > self.tolerance && iter < max_iter {
            let c = (self.a + self.b) / 2.0;
            if (self.function)(c) == 0.0 {
                return (c, error_abs, iter);
            } else if (self.function)(self.a) * (self.function)(c) < 0.0 {
                self.b = c;
            } else {
                self.a = c;
            }

            iter += 1;
        }

        ((self.a + self.b) / 2.0, error_abs, iter)
    }
}

impl FindRoot for FalsePosition {
    fn info() {
        println!("Welcome to False Position Method");
        println!("You  need to provide:");
        println!("1. A function (f)");
        println!("2. An interval (a, b) in which the function will be evaluated in search of the roots.");
        println!("3. Tolerance (tol).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64) -> Self {
        FalsePosition {
            function,
            a,
            b,
            tolerance,
        }
    }

    fn solve(&mut self) -> (f64, f64, usize) {
        let max_iter = (f64::log10((self.b - self.a) / self.tolerance) / f64::log(2.0, std::f64::consts::E)).ceil() as usize;
        let mut iter = 0;
        let mut x0 = self.a;
        let mut x1 = self.b;
        let mut error_abs = ((self.function)(x1) *(x1-x0)).abs();

        while iter < max_iter {
            let f0 = (self.function)(x0);
            let f1 = (self.function)(x1);

            let x2 = x1 - (f1 * (x1 - x0)) / (f1 - f0);

            if error_abs < self.tolerance {
                return (x2, error_abs, iter);
            }

            if (self.function)(x2) * f1 < 0.0 {
                x0 = x2;
            } else {
                x1 = x2;
            }
            iter += 1
        }

        (x1, error_abs, iter)
    }
}

impl FindRoot for ITP {
    fn info() {
        println!("Welcome to Interpolate Truncate and Project (ITP) Method");
        println!("You  need to provide:");
        println!("1. A function (f)");
        println!("2. An interval (a, b) in which the function will be evaluated in search of the roots.");
        println!("3. Tolerance (tol).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64) -> Self {
        ITP {
            function,
            a,
            b,
            tolerance,
        }
    }

    fn solve(&mut self) -> (f64, f64, usize) {
        let max_iter = f64::log2((self.a - self.b) / (2.0 * self.tolerance)).ceil() as usize;

        let mut iter = 0;
        let mut x0 = self.a;
        let mut x1 = self.b;
        let mut error_abs = (x0 - x1).abs();

        while error_abs > self.tolerance && iter < max_iter {
            let f0 = (self.function)(x0);
            let f1 = (self.function)(x1);

            let m = (f1 - f0) / (x1 - x0);
            let x2 = x1 - f1 / m;

            let y0 = (self.function)(x0);
            let y2 = (self.function)(x2);
            let mut xt = x2 - (y2 - y0) / (2.0 * m);

            xt = xt.round();
            x0 = x1;
            x1 = xt;
            iter += 1;
        }

        (x1, error_abs, iter)
    }
}
