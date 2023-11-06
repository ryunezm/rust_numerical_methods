pub struct Bisection{
    function: fn(f64) -> f64,
    a: f64,
    b: f64,
    tolerance: f64
}

impl Bisection{

    pub fn new(function: fn(f64)->f64, a: f64, b: f64, tolerance: f64) -> Bisection{
        Bisection {function, a, b, tolerance}
    }

    pub fn solve(&mut self) -> f64{
        while(self.a - self.b).abs() > self.tolerance {
            let c = (self.a + self.b)/2.0;
            if (self.function)(c) == 0.0 {
                return c;
            } else if (self.function)(self.a) * (self.function)(c) < 0.0 {
                self.b = c;
            } else {
                self.a = c;
            }
        }

        return (self.a + self.b) / 2.0;
    }
}
