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

    pub fn get_distance_from_point(&self, point: &Point) -> f32 {
        let a = Self::delta(point.x, self.x);
        let b = Self::delta(point.y, self.y);

        (b.powi(2) + a.powi(2)).sqrt()
    }

    fn delta(b: f32, a: f32) -> f32 {
        (b-a).abs()
    }

    pub fn delta_from_point(&self, point: &Point) -> Point {
        Point {
            x: Point::delta(point.x, self.x),
            y: Point::delta(point.y, self.y),
        }
    }

    pub fn dot_product(&self, point: &Point) -> f32 {
        self.x * point.x + self.y * point.y
    }

    pub fn rotate_around_origin(&self, theta: f32) -> Point {
        let cos = (theta.to_radians()).cos();
        let sin = (theta.to_radians()).sin();

        // Result of trig functions will not be exact
        let safe_cos = if (cos).abs() <= 0.01 { 0.0 } else { cos };
        let safe_sin = if (sin).abs() <= 0.01 { 0.0 } else { sin };

        Point {
            x: self.x * safe_cos + self.y * safe_sin,
            y: -self.x * safe_sin + self.y * safe_cos,
        }
    }
}