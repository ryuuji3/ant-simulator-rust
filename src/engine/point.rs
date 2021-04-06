#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn create_point(x: f32, y: f32) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn get_distance_from_point(&self, point: Point) -> f32 {
        let a = Self::delta(point.x, self.x);
        let b = Self::delta(point.y, self.y);

        (b.powi(2) + a.powi(2)).sqrt()
    }

    fn delta(b: f32, a: f32) -> f32 {
        (b-a).abs()
    }

    pub fn delta_from_point(&self, point: Point) -> Point {
        Point {
            x: Point::delta(point.x, self.x),
            y: Point::delta(point.y, self.y),
        }
    }

    pub fn dot_product(&self, point: Point) -> f32 {
        self.x * point.x + self.y * point.y
    }
}