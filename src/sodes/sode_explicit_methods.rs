// ------ TRAIT ------
pub(crate) trait Functions{
    fn info();
    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: i64) -> Self;
    fn solve(&mut self) -> Option<f64>;

}

// ------ STRUCTS ------
pub struct Euler {
    function: Box<dyn Fn(f64, f64) -> f64>,
    y0: f64,
    h: f64,
    n: i64,
}

pub struct RK2 {
    function: Box<dyn Fn(f64, f64) -> f64>,
    y0: f64,
    h: f64,
    n: i64,
}

pub struct RK4 {
    function: Box<dyn Fn(f64, f64) -> f64>,
    y0: f64,
    h: f64,
    n: i64,
}

// ------ IMPLEMENTATIONS ------

impl Functions for Euler{
    fn info() {
        println!("Welcome to Euler's method");
        println!("You  need to provide:");
        println!("1. A function (f(x,y)).");
        println!("2. Initial value of the dependent variable (y0).");
        println!("3. Step size (h).");
        println!("4. Number of steps or iterations (n).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: i64) -> Self {
        Euler{ function, y0, h, n }
    }

    fn solve(&mut self) -> Option<f64> {
        for i in 0..self.n {
            self.y0 += self.h * (self.function)(self.y0, i as f64);
        }
        Some(self.y0)
    }
}

impl Functions for RK2{
    fn info() {
        println!("Welcome to Runge-Kutta-2 Method (Midpoint Method)");
        println!("You  need to provide:");
        println!("1. A function (f(x,y)).");
        println!("2. Initial value of the dependent variable (y0).");
        println!("3. Step size (h).");
        println!("4. Number of steps or iterations (n).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: i64) -> Self {
        RK2{ function, y0, h, n }
    }

    fn solve(&mut self) -> Option<f64> {
        for i in 0..self.n {

            let k1 = self.h * (self.function)(self.y0, i as f64);
            let k2 = self.h * (self.function)(self.y0 + k1 / 2.0, i as f64 + self.h / 2.0);
            self.y0 += k2;
        }
        Some(self.y0)
    }
}

impl Functions for RK4 {
    fn info() {
        println!("Welcome to Runge-Kutta-4 Method (Classical Fourth Order Method)");
        println!("You  need to provide:");
        println!("1. A function (f(x,y)).");
        println!("2. Initial value of the dependent variable (y0).");
        println!("3. Step size (h).");
        println!("4. Number of steps or iterations (n).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: i64) -> Self {
        RK4{ function, y0, h, n }
    }

    fn solve(&mut self) -> Option<f64> {
        for i in 0..self.n {
            let k1 = self.h * (self.function)(self.y0, i as f64);
            let k2 = self.h * (self.function)(self.y0 + k1 / 2.0, i as f64 + self.h / 2.0);
            let k3 = self.h * (self.function)(self.y0 + k2 / 2.0, i as f64 + self.h / 2.0);
            let k4 = self.h * (self.function)(self.y0 + k3, i as f64 + self.h);
            self.y0 += (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        }
        Some (self.y0)
    }
}