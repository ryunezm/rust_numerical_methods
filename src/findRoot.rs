// Trait
trait FindRoot {
    fn new(function: fn(f64) -> f64, a: f64, b: f64, tolerance: f64) -> Self;
    fn solve(&mut self) -> f64;
}

// Structs
pub struct Bisection {
    function: fn(f64) -> f64,
    a: f64,
    b: f64,
    tolerance: f64,
}

pub struct FalsePosition{
    function: fn(f64) -> f64,
    a: f64,
    b: f64,
    tolerance: f64,
    max_iter: u32
}

pub struct ITP{
    function: fn(f64) -> f64,
    a: f64,
    b: f64,
    tolerance: f64,
}

// Implementations
impl FindRoot for Bisection {
    fn new(function: fn(f64) -> f64, a: f64, b: f64, tolerance: f64) -> Self {
        Bisection {
            function,
            a,
            b,
            tolerance,
        }
    }

    fn solve(&mut self) -> f64 {
        while (self.b - self.a).abs() > self.tolerance {
            let c = (self.a + self.b) / 2.0;
            if (self.function)(c) == 0.0 {
                return c;
            } else if (self.function)(self.a) * (self.function)(c) < 0.0 {
                self.b = c;
            } else {
                self.a = c;
            }
        }

        (self.a + self.b) / 2.0
    }
}

impl FindRoot for ITP{
    fn new(function: fn(f64) -> f64, a: f64, b: f64, tolerance: f64) -> Self {
        ITP{
            function,
            a,
            b,
            tolerance
        }
    }

    fn solve(&mut self) -> f64 {
        let mut x0 = self.a;
        let mut x1 = self.b;

        while (x0 - x1).abs() > self.tolerance{
            let f0 = (self.function)(self.a);
            let f1 = (self.function)(self.b);

            let m = (f1-f0)/(x1-x0);
            let x2 = x1 - f1/m;

            let y0 = (self.function)(self.a);
            let y2 = (self.function)(x2);
            let xt = x2-(y2-y0)/(2*m);

            xt = xt.round();
            x0 = x1;
            x1 = xt;

        }
        x1

    }
}