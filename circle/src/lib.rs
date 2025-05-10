#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self { center: Point(x, y), radius: r }
    }

    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(self) -> f64 {
        self.radius.powi(2) * std::f64::consts::PI
    }

    pub fn intersect(self, circle2: Circle) -> bool {
        let rigth = self.radius + circle2.radius;
        let d = self.center.distance(circle2.center);
        if rigth >= d  {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    // distance is √[(x2 − x1)2 + (y2 − y1)2]
    pub fn distance(self, point_b: Point) -> f64 {
        ((point_b.0 - self.0).powf(2.0) + (point_b.1 - self.1).powf(2.0)).sqrt()
    }
}
