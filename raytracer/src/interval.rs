#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn _default() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn two_interval(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Self { min, max }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
    pub fn expand(&self, delta: f64) -> Self {
        Self {
            min: self.min - delta / 2.0,
            max: self.max + delta / 2.0,
        }
    }
    pub fn _empty() -> Self {
        Interval::new(f64::INFINITY, -f64::INFINITY)
    }
    pub fn universe() -> Self {
        Interval::new(-f64::INFINITY, f64::INFINITY)
    }
}
