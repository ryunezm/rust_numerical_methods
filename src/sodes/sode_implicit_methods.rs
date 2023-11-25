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
        println!("Welcome to Implicit Euler's Method");
        println!("You  need to provide:");
        println!("1. A function (f(x,y)).");
        println!("2. Initial value of the dependent variable (y0).");
        println!("3. Step size (h).");
        println!("4. Number of steps or iterations (n).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self {
        ImplicitEuler { function, y0, h, n }
    }

    fn solve(&mut self) -> Option<(f64)> {
        for i in 0..self.n {
            let delta_y = self.h * self.function(self.y0, i + 1.0);
            self.y0 += delta_y / (1.0 - self.h);
        }
        Some(self.y0)
    }
}

impl Functions for BackwardEuler {
    fn info() {
        println!("Welcome to Backward Euler's Method");
        println!("You  need to provide:");
        println!("1. A function (f(x,y)).");
        println!("2. Initial value of the dependent variable (y0).");
        println!("3. Step size (h).");
        println!("4. Number of steps or iterations (n).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self {
        BackwardEuler { function, y0, h, n }
    }

    fn solve(&mut self) -> Option<(f64)> {
        for i in 0..self.n {
            let delta_y = self.h * self.function(self.y0 + self.h, i + 1.0);
            self.y0 += delta_y;
        }
        Some(self.y0)
    }
}

impl Functions for CrankNicolson {
    fn info() {
        println!("Welcome to Crank-Nicolson Method");
        println!("You  need to provide:");
        println!("1. A function (f(x,y)).");
        println!("2. Initial value of the dependent variable (y0).");
        println!("3. Step size (h).");
        println!("4. Number of steps or iterations (n).");
        println!("Please enter the data:")
    }

    fn new(function: Box<dyn Fn(f64, f64) -> f64>, y0: f64, h: f64, n: f64) -> Self {
        CrankNicolson { function, y0, h, n }
    }

    fn solve(&mut self) -> Option<(f64)> {
        for i in 0..self.n {
            let delta_y1 = self.h * self.function(self.y0, i);
            let delta_y2 = self.h * self.function(self.y0 + delta_y1, i + 1.0);
            self.y0 += (delta_y1 + delta_y2) / 2.0;
        }
        Some(self.y0)
    }
}