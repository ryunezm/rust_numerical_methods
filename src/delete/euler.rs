pub struct Euler{
    function: fn(f64) -> f64,
    y0: f64,
    t0: f64,
    h: f64,
    num_steps: u32
}

impl Euler{

    pub fn new(function: fn(f64)->f64, y0: f64, t0: f64,h: f64, num_steps: u32) -> Euler{
        Euler {function, y0, t0, h, num_steps}
    }
    fn solve(&mut self) -> Vec<(f64, f64)>{
        let mut results = Vec::with_capacity(self.num_steps as usize + 1);
        let mut t = self.t0;
        let mut y = self.y0;

        results.push((t, y));

        for _ in 0..self.num_steps {
            let y_new = y + self.h * (self.function)(t, y);
            t += self.h;
            y = y_new;
            results.push((t, y));
        }

        return results;
    }
}