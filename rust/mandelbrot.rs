use pyo3::prelude::*;

#[pyclass]
pub struct ComplexNumber {
    x: f32,
    y: f32,
}

#[pymethods]
impl ComplexNumber {
    #[new]
    fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
    fn add(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn square(&self) -> ComplexNumber {
        ComplexNumber {
            x: self.x * self.x - self.y * self.y,
            y: 2.0 * self.x * self.y,
        }
    }
    fn abs(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
    fn __repr__(&self) -> String {
        if self.y == 0. {
            return format!("{}", self.x);
        }
        if self.y < 0. {
            return format!("{} - {}i", self.x, self.y);
        }
        return format!("{} + {}i", self.x, self.y);
    }
    fn member_of_mandelbrot(&self, max_iter: u8) -> bool {
        let mut z = ComplexNumber { x: 0., y: 0. };
        for _ in 0..max_iter {
            z = z.square().add(&self);
            if z.abs() >= 4. {
                return false;
            }
        }
        true
    }
}

#[pyfunction]
pub fn mandelbrot_set(delta: f32, max_iter: u8) -> Vec<(f32, f32)> {
    let mut res = vec![];
    let mut x = -2.0;
    while x <= 0.5 {
        let mut y = -1.5;
        while y <= 1.5 {
            let c = ComplexNumber {
                x: x.clone(),
                y: y.clone(),
            };
            if c.member_of_mandelbrot(max_iter) {
                res.push(c);
            }
            y = y + delta;
        }
        x = x + delta;
    }
    res.into_iter().map(|r| (r.x, r.y)).collect()
}
