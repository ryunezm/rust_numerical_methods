// ------ TRAIT ------
pub(crate) trait FindRoot {
    fn info();
    fn new(function: Box<dyn Fn(f64) -> f64>, a: f64, b: f64, tolerance: f64, max_iter: usize) -> Self;
    fn solve(&mut self) -> Option<(f64, f64, usize)>;
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
        Brent{ function, a, b, tolerance, max_iter }
    }

    fn solve(&mut self) -> Option<(f64, f64, usize)> {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.b;
        let mut d = 0.0;
        let mut e = 0.0;

        let mut fa = (self.function)(a);
        let mut fb = (self.function)(b);
        let mut fc = fb;

        for iter in 0..self.max_iter {
            if (fb > 0.0 && fc > 0.0) || (fb < 0.0 && fc < 0.0) {
                c = a;
                fc = fa;
                e = b - a;
                d = e;
            }

            if fc.abs() < fb.abs() {
                a = b;
                b = c;
                c = a;
                fa = fb;
                fb = fc;
                fc = fa;
            }

            let tol = 2.0 * f64::EPSILON * b.abs() + 0.5 * self.tolerance;
            let m = 0.5 * (c - b);

            if fb.abs() < tol || (fb == 0.0) {
                return Some((b, fb, iter));
            }

            if e.abs() >= tol && fa.abs() > fb.abs() {
                let s = fb / fa;

                let p = if a == c {
                    2.0 * m * s
                } else {
                    let r = fb / fc;
                    let q = fa / fc;
                    s * (2.0 * m * q * (q - r) - (b - a) * (r - 1.0))
                        / ((q - 1.0) * (r - 1.0) * (s - 1.0))
                };

                if p > 0.0 {
                    e = d;
                    d = p.min(m);
                } else {
                    d = m;
                    e = d;
                }
            } else {
                d = m;
                e = d;
            }

            a = b;
            fa = fb;

            if d.abs() > tol {
                b += d;
            } else {
                b += m.signum() * tol;
            }

            fb = (self.function)(b);
        }

        None
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
        Ridder{ function, a, b, tolerance, max_iter }
    }

    fn solve(&mut self) -> Option<(f64, f64, usize)> {
        let mut a = self.a;
        let mut b = self.b;

        for iter in 0..self.max_iter {
            //let fa = (self.function)(a);
            let fb = (self.function)(b);
            let c = 0.5 * (a + b);
            let fc = (self.function)(c);

            let d = c + 0.25 * (c - a) * (fc - fb).signum();
            let fd = (self.function)(d);

            let e = b - a;
            let m = 0.5 * (a + b);
            let tol = 2.0 * f64::EPSILON * m.abs() + 0.5 * self.tolerance;

            if e.abs() < tol || fc.abs() < self.tolerance {
                return Some((c, fc, iter));
            }

            let p = if (a - c) * fc > 0.0 {
                if (a - c) * fd > 0.0 {
                    d
                } else {
                    c
                }
            } else {
                if (c - b) * fd > 0.0 {
                    d
                } else {
                    c
                }
            };

            let q = if (c - b) * fc > 0.0 {
                if (c - b) * fd > 0.0 {
                    d
                } else {
                    c
                }
            } else {
                if (b - a) * fd > 0.0 {
                    d
                } else {
                    c
                }
            };

            let r = p - (p - a) * fc / (fc - fb);
            let s = q - (q - b) * fd / (fd - fc);

            let (r, s) = if r < s { (r, s) } else { (s, r) };

            if r > a && r < b && (r - a).abs() > 0.5 * e && (b - r).abs() > 0.5 * e {
                return Some((r, (self.function)(r), iter));
            } else if s > a && s < b && (s - a).abs() > 0.5 * e && (b - s).abs() > 0.5 * e {
                return Some((s, (self.function)(s), iter));
            }

            let next = if fc.abs() < fd.abs() { c } else { d };

            if next < c {
                b = c;
            } else {
                a = c;
            }
        }

        None
    }
}

