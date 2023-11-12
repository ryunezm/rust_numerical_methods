// ------ TRAIT ------
pub(crate) trait FindRoot {
    fn info();
    fn solve(&mut self) -> (f64, usize);
}
// ------ STRUCTS ------

struct Newton<'a> {
    function: &'a dyn Fn(f64) -> f64,
    function_derivative: fn(f64) -> f64,
    x0: f64,
    tolerance: f64,
    max_iter: usize,
}

struct Secant<'a> {
    function: &'a dyn Fn(f64) -> f64,
    x0: f64,
    x1: f64,
    tolerance: f64,
    max_iter: usize,
}

struct Steffensen<'a> {
    function: &'a dyn Fn(f64) -> f64,
    x0: f64,
    tolerance: f64,
    max_iter: usize,
}

struct FixedPoint<'a> {
    function: &'a dyn Fn(f64) -> f64,
    x0: f64,
    tolerance: f64,
    max_iter: usize,
}

struct InverseQuadraticInterpolation<'a> {
    function: &'a dyn Fn(f64) -> f64,
    x0: f64,
    x1: f64,
    x2: f64,
    tolerance: f64,
    max_iter: usize,
}

// ------ IMPLEMENTATIONS ------

impl<'a> Newton<'a> {
    fn new(
        function: &'a dyn Fn(f64) -> f64,
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

impl<'a> FindRoot for Newton<'a> {
    fn info() {
        todo!()
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

impl<'a> Secant<'a> {
    fn new(function: &'a dyn Fn(f64) -> f64, x0: f64, x1: f64, tolerance: f64, max_iter: usize) -> Self {
        Secant {
            function,
            x0,
            x1,
            tolerance,
            max_iter,
        }
    }
}

impl<'a> FindRoot for Secant<'a> {
    fn info() {
        todo!()
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

impl<'a> Steffensen<'a> {
    fn new(function: &'a dyn Fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize) -> Self {
        Steffensen {
            function,
            x0,
            tolerance,
            max_iter,
        }
    }
}

impl<'a> FindRoot for Steffensen<'a> {
    fn info() {
        todo!()
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

impl<'a> FixedPoint<'a> {
    fn new(function: &'a dyn Fn(f64) -> f64, x0: f64, tolerance: f64, max_iter: usize) -> Self {
        FixedPoint {
            function,
            x0,
            tolerance,
            max_iter,
        }
    }
}

impl<'a> FindRoot for FixedPoint<'a> {
    fn info() {
        todo!()
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

impl<'a> InverseQuadraticInterpolation<'a> {
    fn new(
        function: &'a dyn Fn(f64) -> f64,
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

impl<'a> FindRoot for InverseQuadraticInterpolation<'a> {
    fn info() {
        todo!()
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
